use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::{broadcast, RwLock};

use crate::fleet_manager::{FleetManager, FleetSnapshot};

#[derive(Clone)]
pub struct AppState {
    pub fleet: Arc<RwLock<FleetManager>>,
    pub broadcaster: broadcast::Sender<FleetSnapshot>,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| client(socket, state))
}

async fn client(mut socket: WebSocket, state: AppState) {
    let initial_snapshot = {
        let guard = state.fleet.read().await;
        guard.snapshot()
    };

    if send_snapshot(&mut socket, &initial_snapshot).await.is_err() {
        return;
    }

    let mut subscription = state.broadcaster.subscribe();
    while let Ok(snapshot) = subscription.recv().await {
        if send_snapshot(&mut socket, &snapshot).await.is_err() {
            return;
        }
    }
}

async fn send_snapshot(socket: &mut WebSocket, snapshot: &FleetSnapshot) -> Result<(), axum::Error> {
    let payload = serde_json::to_string(snapshot).expect("snapshot serialization should succeed");
    socket.send(Message::Text(payload.into())).await
}
