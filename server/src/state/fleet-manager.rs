use std::{collections::HashMap, time::{Duration, Instant}};

use serde::{Deserialize, Serialize};

fn default_status() -> String {
    "nominal".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetrySample {
    pub node_id: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
    #[serde(default)]
    pub temperature_c: f32,
    #[serde(default)]
    pub signal_dbm: i32,
    #[serde(default = "default_status")]
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetNode {
    pub node_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub temperature_c: f32,
    pub signal_dbm: i32,
    pub status: String,
    pub age_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetSnapshot {
    pub node_count: usize,
    pub nodes: Vec<FleetNode>,
}

#[derive(Debug)]
struct NodeState {
    node_id: String,
    x: f32,
    y: f32,
    z: f32,
    temperature_c: f32,
    signal_dbm: i32,
    status: String,
    last_seen: Instant,
}

#[derive(Debug, Default)]
pub struct FleetManager {
    nodes: HashMap<String, NodeState>,
}

impl FleetManager {
    pub fn update(&mut self, sample: TelemetrySample) {
        let node = NodeState {
            node_id: sample.node_id.clone(),
            x: sample.x,
            y: sample.y,
            z: sample.z,
            temperature_c: sample.temperature_c,
            signal_dbm: sample.signal_dbm,
            status: sample.status,
            last_seen: Instant::now(),
        };

        self.nodes.insert(sample.node_id, node);
    }

    pub fn prune_stale(&mut self, max_age: Duration) {
        self.nodes
            .retain(|_, node| node.last_seen.elapsed() <= max_age);
    }

    pub fn snapshot(&self) -> FleetSnapshot {
        let mut nodes = self
            .nodes
            .values()
            .map(|node| FleetNode {
                node_id: node.node_id.clone(),
                x: node.x,
                y: node.y,
                z: node.z,
                temperature_c: node.temperature_c,
                signal_dbm: node.signal_dbm,
                status: node.status.clone(),
                age_ms: node.last_seen.elapsed().as_millis(),
            })
            .collect::<Vec<_>>();

        nodes.sort_by(|left, right| left.node_id.cmp(&right.node_id));

        FleetSnapshot {
            node_count: nodes.len(),
            nodes,
        }
    }
}
