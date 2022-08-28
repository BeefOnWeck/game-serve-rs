//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-chat
//! ```

mod games;
use games::hexagon::HexagonIsland;
use crate::games::core::traits::Game;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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
use serde_json::to_string;

// Our shared state
struct AppState {
    tx: broadcast::Sender<String>,
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

    let (tx, _rx) = broadcast::channel(100);
    let game = Mutex::new(HexagonIsland::new());

    let app_state = Arc::new(AppState { tx, game });

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
    let (mut ws_sender, mut ws_receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    let mut key = String::new();

    // Loop until a text message is found.
    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(name) = message {

            username = String::from(&name);

            let error = add_player(&state, &name);
            match error {
                Ok(val) => {
                    key = val;
                    break;
                },
                Err(msg) => {
                    let _ = ws_sender
                        .send(Message::Text(String::from(msg)))
                        .await;

                    return;
                }
            }
        }
    }

    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // Send joined message to all subscribers.
    // let msg = format!("{} joined.", username);
    let msg = serialize_game_status(&state, &key);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // TODO: This message will contain the updated state. We need to customize this for each player and then send it via WS.
            // In any websocket error, break loop.
            if ws_sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{}: {}", name, text));
            // TODO: We receive a WS message from a client, process that command to update state, and then send that updated state to each task.
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // So the Rx and Tx loops run continuously.
    // If one of them exits then the other is stopped.
    // Then you would proceed to here, where the user is removed.

    // Will have to add ping/pong messages to ensure connnections are alive.
    // Try to automatically reconnect a user if we end up here.

    // Send user left message.
    let msg = format!("{} left.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

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

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../assets/index.html"))
}