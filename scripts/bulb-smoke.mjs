// Smoke tests for the solid-fractal (Mandelbulb/Mandelbox) path:
// WASM DE field contract + slab composition + a real headless MarchingCubes
// isosurface build (no WebGL). Also times the shipped grid.
// Usage: node scripts/bulb-smoke.mjs path/to/mandelbrot.wasm
import { readFileSync, writeFileSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";

const { instance } = await WebAssembly.instantiate(readFileSync(process.argv[2]));
const e = instance.exports;

function fieldView(g) {
  return new Float32Array(e.memory.buffer, e.field_ptr(), g * g * g);
}

// --- slab composition + sign structure (Mandelbulb p8) ---
const G = 32;
e.bulb_field(0, G, 14, 0, G);
const whole = fieldView(G).slice(0, G * G * G);
e.bulb_field(0, G, 14, 0, 16);
e.bulb_field(0, G, 14, 16, G);
const split = fieldView(G).slice(0, G * G * G);
if (whole.length !== split.length || whole.some((v, i) => v !== split[i])) {
  throw new Error("slab composition != single call");
}
const f32 = fieldView(G);
const f = (x, y, z) => f32[z * G * G + y * G + x];
if (f(16, 16, 16) >= 0) throw new Error("origin not inside");
if (f(0, 0, 0) <= 0) throw new Error("corner not outside");
let signChange = 0;
for (let k = 0; k < G * G * G; k++) if (whole[k] === 0) signChange++;
if (signChange > 0) throw new Error("exact-zero field values (sign loss)");
console.log("field contract + slab composition OK (p8)");

// Mandelbox: interior/exterior mix
e.bulb_field(5, G, 12, 0, G);
const box = fieldView(G);
// Mandelbox s3 is a sparse lacework; require some structure, not a bulk solid.
let inside = 0, outside = 0;
for (let k = 0; k < G * G * G; k++) { if (box[k] < 0) inside++; else if (box[k] > 0) outside++; }
if (inside < 50 || outside < 1000) throw new Error(`box mix bad: in ${inside} out ${outside}`);
if (!box.every((v) => Number.isFinite(v))) throw new Error("non-finite box field");
console.log(`mandelbox contract OK: ${inside} inside, ${outside} outside`);

// --- headless MarchingCubes isosurface from the real field ---
// Rewrite the bare 'three' specifier so Node can import it without node_modules.
const mcSrc = readFileSync("web/vendor/objects/MarchingCubes.js", "utf8")
  .replace("from 'three'", "from '../web/vendor/three.module.min.js'");
const tmpUrl = new URL("./.tmp-mc.mjs", import.meta.url);
writeFileSync(fileURLToPath(tmpUrl), mcSrc);
const { MarchingCubes } = await import(tmpUrl.href);
rmSync(fileURLToPath(tmpUrl));

const GRID = 64; // shipped Standard
e.bulb_field(0, GRID, 14, 0, GRID);
const mc = new MarchingCubes(GRID, { flatShading: false }, false, false, 600000);
mc.isolation = 0;
mc.field.set(fieldView(GRID));
const t0 = performance.now();
mc.update();
const ms = performance.now() - t0;
if (mc.count <= 0) throw new Error("isosurface empty");
let bad = 0;
for (let k = 0; k < mc.count * 3; k++) if (!Number.isFinite(mc.normalArray[k])) bad++;
if (bad) throw new Error(`${bad} non-finite normals`);
console.log(`MC isosurface OK @ ${GRID}³: ${mc.count} verts, update ${ms.toFixed(0)} ms, normals finite`);

// timing for all grids (bulb p8 top cost)
for (const g of [32, 48, 64, 96]) {
  const t = performance.now();
  e.bulb_field(0, g, 14, 0, g);
  console.log(`  field ${g}³: ${(performance.now() - t).toFixed(0)} ms, verts @96 = ${(() => { e.bulb_field(0,96,14,0,96); const m=new MarchingCubes(96, { flatShading: false }, false, false, 600000); m.isolation=0; m.field.set(fieldView(96)); m.update(); return m.count; })()}`);
}
console.log("BULB SMOKE OK");
