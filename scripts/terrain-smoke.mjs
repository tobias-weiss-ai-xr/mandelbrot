// Smoke tests for the 3D terrain path: WASM iteration-buffer contract +
// the terrain.html geometry/z-height math (three.js imported headless, no WebGL).
// Usage: node scripts/terrain-smoke.mjs path/to/mandelbrot.wasm
import { readFileSync } from "node:fs";
import * as THREE from "../web/vendor/three.module.min.js";

const W = 960, H = 640, H_MAX = 2.5;

const { instance } = await WebAssembly.instantiate(readFileSync(process.argv[2]));
const e = instance.exports;

// --- iters buffer contract (scale 2, then agreement with scale 1) ---
e.render(0, -0.6, 0, 2.7, 256, 2);
const w = W / 2, h = H / 2;
const iters = new Uint32Array(e.memory.buffer, e.iters_ptr(), W * H);
const colors = new Uint32Array(e.memory.buffer, e.buf_ptr(), W * H);
for (let k = 0; k < w * h; k++) {
  if (iters[k] > 256) throw new Error(`iters[${k}] = ${iters[k]} > 256`);
  if ((iters[k] === 256) !== (colors[k] === 0xFF000000)) throw new Error(`classification mismatch at ${k}`);
}
const samples = [];
for (let j = 0; j < h; j += 53) for (let i = 0; i < w; i += 59) samples.push([j * w + i, iters[j * w + i]]);
e.render(0, -0.6, 0, 2.7, 256, 1);
for (const [idx, expected] of samples) {
  const j = (idx / w) | 0, i = idx % w;
  if (iters[j * 2 * W + i * 2] !== expected) throw new Error("scale1/scale2 iters disagree");
}
console.log("iters buffer contract OK");

// --- terrain math against real three.js classes ---
const vw = { iters: 256 };
e.render(3, -1.75, -0.03, 0.15, vw.iters, 4); // burning ship at coarse-ish detail
const tw = W / 4, th = H / 4;
const geo = new THREE.PlaneGeometry(16, 16 * th / tw, tw - 1, th - 1);
geo.setAttribute("color", new THREE.BufferAttribute(new Float32Array(tw * th * 3), 3));
const it4 = new Uint32Array(e.memory.buffer, e.iters_ptr(), tw * th);
const pos = geo.attributes.position, col = geo.attributes.color;
let interior = 0, elevated = 0, zMin = 1e9, zMax = -1e9;
for (let k = 0; k < tw * th; k++) {
  const z = it4[k] >= vw.iters ? 0 : Math.sqrt(it4[k] / vw.iters) * H_MAX;
  pos.setZ(k, z);
  const c = colors[k];
  col.setXYZ(k, (c & 255) / 255, ((c >> 8) & 255) / 255, ((c >> 16) & 255) / 255);
  if (z === 0) interior++; else { elevated++; if (z < zMin) zMin = z; if (z > zMax) zMax = z; }
}
if (interior === 0 || elevated === 0) throw new Error("terrain has no interior/exterior mix");
if (zMax <= zMin || zMax > H_MAX + 1e-9) throw new Error(`bad z range [${zMin}, ${zMax}]`);
pos.needsUpdate = true;
geo.computeVertexNormals();
const n = geo.attributes.normal;
let bad = 0;
for (let k = 0; k < n.count; k++) if (!Number.isFinite(n.getX(k) + n.getY(k) + n.getZ(k))) bad++;
if (bad) throw new Error(`${bad} non-finite normals`);
console.log(`terrain math OK: ${tw * th} verts, interior ${interior}, elevated ${elevated}, z [${zMin.toFixed(2)}, ${zMax.toFixed(2)}], normals finite`);
console.log("TERRAIN SMOKE OK");
