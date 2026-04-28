# Spatial IoT Fleet

Spatial IoT Fleet is a three-part telemetry stack for visualizing large sensor swarms in real time:

- `edge-device`: a PlatformIO-friendly C++ simulator that emits UDP telemetry packets.
- `server`: a Rust service that ingests UDP packets, maintains in-memory fleet state, and streams snapshots over WebSockets.
- `web-client`: a Vite and Three.js dashboard that renders the active fleet with `InstancedMesh`.

## Repository Layout

```text
spatial-iot-fleet/
|- edge-device/
|- server/
`- web-client/
```

## Features

- Simulated sensor nodes with deterministic motion and link-quality metadata.
- Rust-backed fleet state tracking with stale-node pruning.
- WebSocket snapshot streaming for live browser updates.
- Three.js scene optimized for large node counts.
- Placeholder `.gltf` asset staging for future model swaps.

## Quick Start

### 1. Start the Rust telemetry service

```bash
cd server
cargo run
```

The service binds UDP on `0.0.0.0:7001` and WebSockets on `0.0.0.0:7002` by default.

### 2. Start the web client

```bash
cd web-client
npm install
npm run dev
```

The dashboard expects the backend WebSocket at `ws://127.0.0.1:7002/ws` unless `VITE_WS_URL` is set.

### 3. Emit telemetry from the edge simulator

Build or run the C++ simulator from `edge-device` with PlatformIO. For local smoke tests, the source is also compatible with a native C++17 toolchain.

## Validation

```bash
cd server && cargo check
cd ../web-client && npm install && npm run build
```

## License

This project is licensed under the [MIT License](LICENSE).

