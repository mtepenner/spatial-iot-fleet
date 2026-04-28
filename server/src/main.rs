use std::{env, error::Error, net::SocketAddr, sync::Arc, time::Duration};

use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::json;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{broadcast, RwLock},
    time::sleep,
};

#[path = "api/websocket.rs"]
mod websocket;
#[path = "network/udp-listener.rs"]
mod udp_listener;
#[path = "state/fleet-manager.rs"]
mod fleet_manager;

use fleet_manager::FleetSnapshot;
use websocket::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let udp_bind = parse_bind("SPATIAL_IOT_UDP_BIND", "0.0.0.0:7001")?;
    let http_bind = parse_bind("SPATIAL_IOT_HTTP_BIND", "0.0.0.0:7002")?;

    let fleet = Arc::new(RwLock::new(fleet_manager::FleetManager::default()));
    let (broadcaster, _) = broadcast::channel::<FleetSnapshot>(64);

    let socket = UdpSocket::bind(udp_bind).await?;
    println!("udp listener bound to {}", socket.local_addr()?);

    let udp_fleet = Arc::clone(&fleet);
    let udp_broadcaster = broadcaster.clone();
    tokio::spawn(async move {
        if let Err(error) = udp_listener::listen(socket, udp_fleet, udp_broadcaster).await {
            eprintln!("udp listener stopped: {error}");
        }
    });

    let prune_fleet = Arc::clone(&fleet);
    let prune_broadcaster = broadcaster.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            let snapshot = {
                let mut guard = prune_fleet.write().await;
                guard.prune_stale(Duration::from_secs(10));
                guard.snapshot()
            };
            let _ = prune_broadcaster.send(snapshot);
        }
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(websocket::handler))
        .with_state(AppState { fleet, broadcaster });

    let listener = TcpListener::bind(http_bind).await?;
    println!("websocket server listening on ws://{}/ws", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

fn parse_bind(variable: &str, fallback: &str) -> Result<SocketAddr, Box<dyn Error>> {
    let bind = env::var(variable).unwrap_or_else(|_| fallback.to_string());
    bind.parse::<SocketAddr>().map_err(|error| error.into())
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let snapshot = {
        let guard = state.fleet.read().await;
        guard.snapshot()
    };

    Json(json!({
        "status": "ok",
        "node_count": snapshot.node_count,
    }))
}
