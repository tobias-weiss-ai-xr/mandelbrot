## Why

Full-resolution frames take 66–140 ms (256 iters, measured), and the UI schedules one full render per zoom/pan frame. Fast wheel or drag input therefore queues a chain of full-res renders and the viewer feels laggy exactly while exploring. The WASM artifact is also shipped unoptimized.

## What Changes

- Renderer supports reduced-resolution renders (scale 2 → 1/4 the pixels) into a compact buffer, for fast interactive previews.
- Renderer hot loop is restructured so the variant dispatch happens once per render, not once per iteration (optional f64x2 SIMD kept only if the benchmark shows ≥1.3×).
- UI renders low-res frames during zoom/pan and schedules exactly one full-resolution render after interaction settles (~150 ms).
- UI shows the last full-res render time in ms (also the verification instrument for this change).
- Node-based benchmark with a relative speedup gate (low-res vs full-res ≥ 2.5×) and an absolute regression fence, wired into CI.
- CI optimizes the wasm artifact with `wasm-opt` before deploying.

## Capabilities

### New Capabilities

### Modified Capabilities
- `fractal-rendering`: adds requirements for reduced-resolution rendering and a render-time performance budget.
- `viewer-ui`: adds requirements for progressive interaction rendering and a render-time readout.
- `pages-deployment`: adds a requirement that the deployed wasm artifact be optimized (binaryen).

## Impact

- `src/lib.rs`: render ABI gains a scale parameter (6th arg); new tests.
- `web/index.html`: interaction/settle logic + ms readout; canvas internal size swaps during preview.
- `.github/workflows/deploy.yml` + `scripts/bench.mjs`: benchmark step, binaryen install, wasm-opt pass.
- No new npm/rust dependencies in the app itself.
