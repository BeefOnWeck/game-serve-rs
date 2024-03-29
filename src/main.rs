use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::{Html, IntoResponse},
    http::StatusCode,
    routing::{get, post},
    Router,
    Form
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use serde_json::from_str;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod games;
use crate::games::core::traits::Game;
use games::hexagon::HexagonIsland;
use games::hexagon::actions::Command;
use games::hexagon::Config;

#[derive(Clone)]
enum BroadcastType {
    Status,
    Error {player_key: String, message: String}
}

// Our shared state
struct AppState {
    producer: broadcast::Sender<BroadcastType>,
    game: Mutex<HexagonIsland>
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_chat=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (producer, _listener) = broadcast::channel(100);
    let game = Mutex::new(HexagonIsland::new());

    let app_state = Arc::new(AppState { producer, game });

    // Broadcast the game state at regular intervals
    let cloned_app_state = app_state.clone();
    tokio::spawn(async move {
        loop {
            let _ = cloned_app_state.producer.send(BroadcastType::Status);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/start", post(start_game))
        .route("/websocket", get(websocket_handler))
        .layer(Extension(app_state)); // injecting state into all the above routes

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

#[derive(Deserialize)]
struct Joining {
    name: String,
    key: Option<String>
}

// on upgrade to ws
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting we can send and receive at the same time.
    let (mut ws_tx, mut ws_rx) = stream.split();

    // User key gets set in the receive loop, if it's valid.
    let mut key = String::new();

    // Loop until an initial message is found.
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Message::Text(text) = message {
            match from_str::<Joining>(&text) {
                Ok(joined) => {
                    let Joining {name, key: cached_key} = joined;
                    match cached_key {
                        Some(ckey) => {
                            let game = state.game.lock().unwrap();
                            if game.players.list.iter().any(|p| p.key == ckey) {
                                key = ckey;
                                break;
                            }
                        },
                        None => {}
                    }
                    // Try to add this player to game.
                    let attempt = add_player(&state, &name);
                    match attempt {
                        Ok(val) => {
                            key = val;
                            break;
                        },
                        Err(msg) => {
                            let _ = ws_tx.send(
                                Message::Text(
                                    String::new() +
                                    "{" +
                                    "\"error\": " +
                                    "\"" + msg + "\"" +
                                    "}"
                                )
                            ).await;
                            return;
                        }
                    }
                },
                Err(err) => { // Deserialization error
                    let _ = state.producer.send(BroadcastType::Error { 
                        player_key: key.clone(),
                        message: err.to_string()
                    });
                }

            }
            
        }
    }

    // Subscribe this task to the broadcast channel.
    let mut listener = state.producer.subscribe();

    // Need to make clones that the transmit task will take ownership of.
    let cloned_app_state = state.clone();
    let cloned_key = key.clone();

    // This task will receive broadcast messages and send text message to our client.
    let mut websocket_transmit_task = tokio::spawn(async move {
        while let Ok(broadcast) = listener.recv().await {
            match broadcast {
                BroadcastType::Status => {
                    let serialized = serialize_game_status(&cloned_app_state, &cloned_key);
                    if ws_tx.send(
                        Message::Text(
                            String::new() +
                            "{" +
                            "\"state\": " +
                            &serialized +
                            "}"
                        )
                    ).await.is_err() {
                        // break loop on any websocket error
                        break;
                    }
                },
                BroadcastType::Error {player_key,message} => {
                    if *&cloned_key == player_key { 
                        if ws_tx.send(
                            Message::Text(
                                String::new() +
                                "{" +
                                "\"error\": " +
                                "\"" + &message + "\"" +
                                "}"
                            )
                        ).await.is_err() {
                            // break loop on any websocket error
                            break;
                        }
                    }
                }
            }
        }
    });

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut websocket_receive_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = ws_rx.next().await {
            // Try to deserialize text into a Command struct
            match from_str::<Command>(&text) {
                Ok(cmd) => { // Deserialized Command
                    let attempt = process_command(&state, cmd);
                    match attempt {
                        Ok(_) => {},
                        Err(msg) => {
                            let _ = state.producer.send(BroadcastType::Error { 
                                player_key: key.clone(),
                                message: msg.to_string() 
                            });
                        }
                    }
                },
                Err(err) => { // Deserialization error
                    let _ = state.producer.send(BroadcastType::Error { 
                        player_key: key.clone(),
                        message: err.to_string() //"Error: Malformed command".to_string() //err.to_string() 
                    });
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut websocket_transmit_task) => websocket_receive_task.abort(),
        _ = (&mut websocket_receive_task) => websocket_transmit_task.abort(),
    };

    // So the Rx and Tx loops run continuously.
    // If one of them exits then the other is stopped.

}

fn add_player(state: &AppState, name: &str) -> Result<String,&'static str> {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let mut game = state.game.lock().unwrap();
    let result = game.add_player(&key, name);
    match result {
        Ok(_) => Ok(key),
        Err(msg) => Err(msg)
    }
}

fn serialize_game_status(state: &AppState, key: &str) -> String {
    let game = state.game.lock().unwrap();
    game.get_game_status(key)
}

fn process_command(state: &AppState, cmd: Command) -> Result<(),&'static str> {
    let mut game = state.game.lock().unwrap();
    let result = game.process_action(cmd);
    match result {
        Ok(_) => Ok(()),
        Err(msg) => Err(msg)
    }
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../assets/index.html"))
}

async fn start_game(form: Form<Config>, Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    // Use Form extractor to get configuration that was posted 
    // as application/x-www-form-urlencoded
    let config = form.0;
    let mut game = state.game.lock().unwrap();
    let result = game.reset().configure_game(config);

    match result {
        Ok(_) => (StatusCode::CREATED, "Game started"),
        Err(msg) => (StatusCode::BAD_REQUEST, msg)
    }
}