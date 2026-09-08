// Performance gate: median-of-5 render timings per variant.
// Gates: scale-2 preview ≥ 2.5× faster than full-res (relative, runner-proof),
// full-res Mandelbrot ≤ 250 ms absolute fence, and SIMD/scalar consistency.
// Usage: node scripts/bench.mjs path/to/mandelbrot.wasm
import { readFileSync } from "node:fs";

const W = 960, H = 640;
const VIEWS = [
  { v: 0, cx: -0.6, cy: 0.0, h: 2.7, iters: 256, name: "Mandelbrot" },
  { v: 1, cx: 0.0, cy: 0.0, h: 3.0, iters: 256, name: "Julia" },
  { v: 2, cx: 0.0, cy: 0.0, h: 3.4, iters: 256, name: "Multibrot 3" },
  { v: 3, cx: -1.75, cy: -0.03, h: 0.15, iters: 256, name: "Burning Ship" },
  { v: 4, cx: -0.6, cy: 0.0, h: 2.7, iters: 256, name: "Tricorn" },
];
const FENCE_MS = 250, SPEEDUP_MIN = 2.5, CONSISTENCY_MIN = 0.999;

const { instance } = await WebAssembly.instantiate(readFileSync(process.argv[2]));
const e = instance.exports;
const buf = new Uint32Array(e.memory.buffer, e.buf_ptr(), W * H);

const median = (a) => a.sort((x, y) => x - y)[a.length >> 1];
const time = (scale, view) => {
  const t = [];
  for (let i = 0; i < 5; i++) {
    const t0 = performance.now();
    e.render(view.v, view.cx, view.cy, view.h, view.iters, scale);
    t.push(performance.now() - t0);
  }
  return median(t);
};

const rows = VIEWS.map((vw) => {
  const full = time(1, vw);
  const low = time(2, vw);
  return { name: vw.name, "full ms": full.toFixed(1), "low ms": low.toFixed(1), "speedup": (full / low).toFixed(2) };
});
console.table(rows);

const fenceOk = +rows[0]["full ms"] <= FENCE_MS;
const speedupOk = rows.every((r) => +r.speedup >= SPEEDUP_MIN);

// Consistency: scale-1 (shipped full-res path) vs scale-2 (scalar preview path)
// must agree on shared pixels — same complex point, same value.
const vw = VIEWS[0];
e.render(vw.v, vw.cx, vw.cy, vw.h, vw.iters, 1);
const fullSamples = [];
for (let j = 0; j < H; j += 22) for (let i = 0; i < W; i += 26) fullSamples.push([j * W + i, buf[j * W + i]]);
e.render(vw.v, vw.cx, vw.cy, vw.h, vw.iters, 2);
let agree = 0;
for (const [idx, val] of fullSamples) {
  const j = (idx / W) | 0, i = idx % W;
  if (buf[(j / 2) * (W / 2) + (i / 2)] === val) agree++;
}
const consistency = agree / fullSamples.length;
console.log(`consistency(scale1 vs scale2): ${(consistency * 100).toFixed(2)}%`);

let ok = true;
if (!fenceOk) { console.error(`FAIL: full-res fence ${rows[0]["full ms"]}ms > ${FENCE_MS}ms`); ok = false; }
if (!speedupOk) { console.error(`FAIL: preview speedup < ${SPEEDUP_MIN}× for some variant`); ok = false; }
if (consistency < CONSISTENCY_MIN) { console.error(`FAIL: scale1/scale2 consistency ${consistency}`); ok = false; }
if (!ok) process.exit(1);
console.log("BENCH OK");
