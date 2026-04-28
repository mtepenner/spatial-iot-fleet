const DEFAULT_WS_URL = import.meta.env.VITE_WS_URL || "ws://127.0.0.1:7002/ws";

export function connectFleetSocket(onSnapshot) {
  let closed = false;
  let simulationTimer = null;
  let socket = null;

  const startSimulation = () => {
    if (closed || simulationTimer !== null) {
      return;
    }

    let tick = 0;
    onSnapshot(buildSimulationSnapshot(tick), { mode: "simulated" });
    simulationTimer = window.setInterval(() => {
      tick += 1;
      onSnapshot(buildSimulationSnapshot(tick), { mode: "simulated" });
    }, 1000);
  };

  try {
    socket = new WebSocket(DEFAULT_WS_URL);

    socket.addEventListener("message", (event) => {
      const snapshot = JSON.parse(event.data);
      onSnapshot(snapshot, { mode: "live" });
    });

    socket.addEventListener("open", () => {
      if (simulationTimer !== null) {
        window.clearInterval(simulationTimer);
        simulationTimer = null;
      }
    });

    socket.addEventListener("error", startSimulation);
    socket.addEventListener("close", startSimulation);
  } catch (error) {
    console.warn("websocket connection failed, switching to simulation", error);
    startSimulation();
  }

  return () => {
    closed = true;
    if (simulationTimer !== null) {
      window.clearInterval(simulationTimer);
    }

    if (socket && socket.readyState < WebSocket.CLOSING) {
      socket.close();
    }
  };
}

function buildSimulationSnapshot(tick) {
  const nodeCount = 144;
  const nodes = Array.from({ length: nodeCount }, (_, index) => {
    const phase = (index / nodeCount) * Math.PI * 2 + tick * 0.18;
    const radius = 8 + (index % 12) * 1.9;

    return {
      node_id: `sensor-${String(index + 1).padStart(3, "0")}`,
      x: Math.cos(phase) * radius,
      y: ((index % 7) - 3) * 0.75,
      z: Math.sin(phase) * radius,
      temperature_c: 18 + ((index + tick) % 11),
      signal_dbm: -42 - ((index + tick) % 30),
      status: index % 19 === 0 ? "warning" : "nominal",
      age_ms: 120,
    };
  });

  return {
    node_count: nodes.length,
    nodes,
  };
}
