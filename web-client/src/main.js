import "./style.css";

import { connectFleetSocket } from "./network/socket.js";
import { createEnvironment } from "./scene/Environment.js";
import { SensorGroup } from "./scene/SensorGroup.js";

const canvas = document.querySelector("#fleet-canvas");
const nodeCount = document.querySelector("[data-node-count]");
const streamState = document.querySelector("[data-stream-state]");

const environment = createEnvironment(canvas);
const sensorGroup = new SensorGroup(environment.scene);

async function boot() {
  await sensorGroup.initialize();

  const disconnect = connectFleetSocket((snapshot, meta) => {
    sensorGroup.update(snapshot);
    nodeCount.textContent = String(snapshot.node_count);
    streamState.textContent = meta.mode.toUpperCase();
  });

  function animate() {
    environment.render();
    window.requestAnimationFrame(animate);
  }

  animate();

  window.addEventListener("beforeunload", () => {
    disconnect();
    environment.destroy();
  });
}

boot().catch((error) => {
  console.error("failed to start spatial fleet client", error);
  streamState.textContent = "ERROR";
});
