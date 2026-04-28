import * as THREE from "three";

export function createEnvironment(canvas) {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color("#020817");
  scene.fog = new THREE.Fog("#020817", 48, 120);

  const camera = new THREE.PerspectiveCamera(50, 1, 0.1, 250);
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: true,
    powerPreference: "high-performance",
  });

  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.outputColorSpace = THREE.SRGBColorSpace;

  const hemisphere = new THREE.HemisphereLight("#e0f2fe", "#052e16", 1.35);
  const key = new THREE.DirectionalLight("#f8fafc", 1.8);
  key.position.set(18, 24, 12);

  const grid = new THREE.GridHelper(180, 30, "#1d4ed8", "#0f766e");
  grid.position.y = -2.5;

  const horizon = new THREE.Mesh(
    new THREE.RingGeometry(36, 40, 96),
    new THREE.MeshBasicMaterial({ color: "#155e75", side: THREE.DoubleSide })
  );
  horizon.rotation.x = Math.PI / 2;
  horizon.position.y = -2.4;

  scene.add(hemisphere, key, grid, horizon);

  const startedAt = performance.now();

  function resize() {
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;

    if (width === 0 || height === 0) {
      return;
    }

    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height, false);
  }

  function render() {
    resize();

    const elapsed = (performance.now() - startedAt) / 1000;
    camera.position.set(Math.cos(elapsed * 0.12) * 42, 22, Math.sin(elapsed * 0.12) * 42);
    camera.lookAt(0, 0, 0);
    renderer.render(scene, camera);
  }

  function destroy() {
    renderer.dispose();
  }

  window.addEventListener("resize", resize);
  resize();

  return {
    scene,
    render,
    destroy() {
      window.removeEventListener("resize", resize);
      destroy();
    },
  };
}
