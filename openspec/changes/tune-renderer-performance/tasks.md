## 1. Renderer

- [x] 1.1 Add `scale` param to `render`: render W/scale × H/scale pixels into the first entries of the buffer, same viewport mapping; unit tests (opaque, compact length, same-region content)
- [x] 1.2 Hoist variant dispatch: monomorphized per-variant inner loop functions selected once per render
- [x] 1.3 Bench-driven f64x2 SIMD path (2 pixels/iteration); keep only if ≥1.3× over scalar in the benchmark, else delete

## 2. UI

- [x] 2.1 Progressive interaction: low-res frames (scale 2, canvas internal size swapped) during wheel/drag; 150 ms settle timer → single full-res render
- [x] 2.2 Render-time readout (ms) for the last full-res render

## 3. Benchmark & CI

- [x] 3.1 `scripts/bench.mjs`: median-of-5 timings for scale 1 and 2 on the default views; gates: speedup ≥ 2.5×, full-res fence 250 ms
- [x] 3.2 Workflow: `apt install binaryen` + `wasm-opt -O3` before assembling `site/`; run bench after build, before upload
- [x] 3.3 Record before/after numbers in this change's notes

## 4. Verification

- [x] 4.1 `cargo test`, bench green locally, `openspec validate --all` passes
- [x] 4.2 Commit, push, workflow green, live site renders; manual wheel-spam check shows low-res-then-settle behavior
