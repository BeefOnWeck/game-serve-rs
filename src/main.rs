use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
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
use games::hexagon::HexagonIsland;
use games::hexagon::actions::Command;
use crate::games::core::traits::Game;

#[derive(Clone)]
enum BroadcastType {
    Status,
    Error {player_key: String, message: String}
}

// Our shared state
struct AppState {
    producer: broadcast::Sender<BroadcastType>, // TODO: Change this to an enum (message type): State
    game: Mutex<HexagonIsland>
}

#[tokio::main]
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
        .route("/websocket", get(websocket_handler))
        .layer(Extension(app_state)); // injecting state into all the above routes

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

// on upgrade to ws
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting we can send and receive at the same time.
    let (mut ws_tx, mut ws_rx) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut key = String::new();

    // Loop until a text message is found.
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Message::Text(name) = message {

            let attempt = add_player(&state, &name);
            match attempt {
                Ok(val) => {
                    key = val;
                    break;
                },
                Err(msg) => {
                    let _ = ws_tx.send(Message::Text(String::from(msg))).await;
                    return;
                }
            }
        }
    }

    // Subscribe before sending joined message.
    let mut listener = state.producer.subscribe();

    // Send joined message to all subscribers.
    // let msg = format!("{} joined.", username);

    let cloned_app_state = state.clone();
    let cloned_key = key.clone();

    // This task will receive broadcast messages and send text message to our client.
    let mut websocket_transmit_task = tokio::spawn(async move {
        while let Ok(broadcast) = listener.recv().await {
            match broadcast {
                BroadcastType::Status => {
                    let serialized = serialize_game_status(&cloned_app_state, &cloned_key);
                    let _ = ws_tx.send(Message::Text(serialized)).await;
                },
                BroadcastType::Error {player_key,message} => {
                    if *&cloned_key == player_key { 
                        let _ = ws_tx.send(Message::Text(message)).await;
                    }
                }
            }
            // In any websocket error, break loop.
            // if ws_tx.send(Message::Text(msg)).await.is_err() {
            //     break;
            // }
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
                        message: err.to_string() 
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
    // Then you would proceed to here, where the user is removed.

    // Will have to add ping/pong messages to ensure connnections are alive.
    // Try to automatically reconnect a user if we end up here.

    // Send user left message.
    // let msg = format!("{} left.", username);
    // tracing::debug!("{}", msg);
    // let _ = state.producer.send(msg);

}

fn add_player(state: &AppState, name: &str) -> Result<String,&'static str> {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let mut game = state.game.lock().unwrap();
    let result = game.add_player(&key, name, "socket_id");
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