use std::{error::Error, sync::Arc};

use tokio::{
    net::UdpSocket,
    sync::{broadcast, RwLock},
};

use crate::fleet_manager::{FleetManager, FleetSnapshot, TelemetrySample};

pub async fn listen(
    socket: UdpSocket,
    fleet: Arc<RwLock<FleetManager>>,
    broadcaster: broadcast::Sender<FleetSnapshot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut buffer = [0_u8; 2048];

    loop {
        let (size, _) = socket.recv_from(&mut buffer).await?;
        if size == 0 {
            continue;
        }

        let packet = match std::str::from_utf8(&buffer[..size]) {
            Ok(packet) => packet,
            Err(_) => continue,
        };

        let sample: TelemetrySample = match serde_json::from_str(packet) {
            Ok(sample) => sample,
            Err(error) => {
                eprintln!("discarding malformed telemetry packet: {error}");
                continue;
            }
        };

        let snapshot = {
            let mut guard = fleet.write().await;
            guard.update(sample);
            guard.snapshot()
        };

        let _ = broadcaster.send(snapshot);
    }
}
