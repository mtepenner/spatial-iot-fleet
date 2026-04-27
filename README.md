# Spatial IoT / Sensor Fleet Visualizer (Rust & Three.js)

## Description
`spatial_iot_fleet` is a full-stack, high-performance spatial visualization tool designed for tracking and managing Internet of Things (IoT) hardware. It bridges simulated C++ edge microcontrollers, an ultra-fast Rust backend for telemetry ingestion, and a rich 3D web client powered by Three.js to provide real-time, spatial monitoring of thousands of active nodes.

## Table of Contents
- [Features](#-features)
- [Technologies Used](#%EF%B8%8F-technologies-used)
- [Installation](#%EF%B8%8F-installation)
- [Usage](#-usage)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Features

* **Edge Device Simulation**: Uses C++ and PlatformIO to simulate microcontroller edge nodes that broadcast high-frequency UDP telemetry packets.
* **Rust Telemetry Server**: A highly optimized Rust backend featuring a `udp_listener` for ultra-fast data ingestion and an in-memory graph (`fleet_manager.rs`) to track the state of all active hardware nodes.
* **Real-Time Data Streaming**: Seamlessly streams the fleet's active state to the browser via WebSockets.
* **3D Spatial Visualization**: A front-end WebGL client built with Vanilla JS, Three.js, and Vite. 
* **High-Performance Rendering**: Utilizes Three.js `InstancedMesh` to efficiently render thousands of 3D sensor nodes (via `.gltf` models) without compromising frame rates, dynamically updating their positions and colors based on the telemetry payload.

## 🛠️ Technologies Used
* **Edge / Microcontrollers**: C++, PlatformIO
* **Backend**: Rust, UDP networking, WebSockets (`Cargo`)
* **Frontend**: Vanilla JavaScript, Three.js (WebGL), Vite
* **Networking Protocol**: UDP for ingestion, WebSockets for client streaming

## ⚙️ Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/mtepenner/spatial_iot_fleet.git
   cd spatial_iot_fleet
   ```

2. **Backend Setup (Rust)**:
   Ensure you have Rust and Cargo installed, then build the server:
   ```bash
   cd server
   cargo build --release
   ```

3. **Frontend Setup (Web Client)**:
   Navigate to the web client directory and install dependencies:
   ```bash
   cd ../web_client
   npm install
   ```

4. **Edge Simulation Setup**:
   Open the `edge_device` folder in PlatformIO to build and flash the C++ firmware, or run the local simulation script if configured.

## 💻 Usage

To launch the spatial tracking environment:

1. **Start the Rust Server**:
   ```bash
   cd server
   cargo run --release
   ```
   *The server will begin listening for UDP packets and initialize the WebSocket stream.*

2. **Start the Web Client**:
   ```bash
   cd web_client
   npm run dev
   ```
   *Open the provided localhost URL in your browser to view the 3D WebGL scene.*

3. **Initialize the Edge Devices**:
   Deploy or run the simulated `main.cpp` code via PlatformIO to begin broadcasting UDP telemetry to the Rust server. You will see the 3D meshes spawn and update in real-time in the browser.

## 🤝 Contributing
Contributions are welcome! Whether you are optimizing the Rust UDP ingestion layer, adding new 3D models to the Three.js scene, or expanding the edge telemetry data structures, please ensure your code maintains the high-performance standards of the project. Submit a pull request with a clear description of your changes.

## 📄 License
This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details. Copyright (c) 2026 Matthew Penner.
