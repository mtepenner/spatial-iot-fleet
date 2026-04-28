import * as THREE from "three";

export class SensorGroup {
  constructor(scene, capacity = 4096) {
    this.scene = scene;
    this.capacity = capacity;
    this.mesh = null;
    this.matrixHelper = new THREE.Object3D();
  }

  async initialize() {
    const geometry = new THREE.IcosahedronGeometry(0.45, 1);
    const material = new THREE.MeshStandardMaterial({
      color: "#34d399",
      roughness: 0.32,
      metalness: 0.08,
    });

    this.mesh = new THREE.InstancedMesh(geometry, material, this.capacity);
    this.mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    this.mesh.count = 0;
    this.scene.add(this.mesh);
  }

  update(snapshot) {
    if (!this.mesh) {
      return;
    }

    const nodes = snapshot.nodes.slice(0, this.capacity);
    this.mesh.count = nodes.length;

    for (let index = 0; index < nodes.length; index += 1) {
      const node = nodes[index];
      const strength = THREE.MathUtils.clamp((Math.abs(node.signal_dbm) - 40) / 30, 0, 1);
      const scale = THREE.MathUtils.mapLinear(node.temperature_c, 14, 34, 0.8, 1.55);
      const color = new THREE.Color().setHSL(0.34 - strength * 0.24, 0.82, 0.52);

      this.matrixHelper.position.set(node.x, node.y, node.z);
      this.matrixHelper.scale.setScalar(scale);
      this.matrixHelper.rotation.set(0, strength * Math.PI * 2, 0);
      this.matrixHelper.updateMatrix();

      this.mesh.setMatrixAt(index, this.matrixHelper.matrix);
      this.mesh.setColorAt(index, node.status === "warning" ? color.offsetHSL(-0.07, 0, 0.04) : color);
    }

    this.mesh.instanceMatrix.needsUpdate = true;
    if (this.mesh.instanceColor) {
      this.mesh.instanceColor.needsUpdate = true;
    }
  }
}
