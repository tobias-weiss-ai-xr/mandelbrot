## Context

Baseline measured this session (Node 22, 256 iters, full 960×640): Mandelbrot 140 ms, Multibrot 103 ms, Julia 73 ms, Tricorn 70 ms, Burning Ship 66 ms. Every zoom/pan frame schedules a full-res render. The hot loop calls a `match`-dispatching `step()` per iteration; the wasm artifact ships without binaryen. GitHub Pages cannot send COOP/COEP headers, so SharedArrayBuffer/wasm-threads are unavailable — single-threaded optimizations only.

## Goals / Non-Goals

**Goals:**
- Instant-feeling zoom/pan via low-res interactive frames; one full-res render on settle.
- Measurable wins on the full-res path: dispatch hoisting, wasm-opt, optional f64x2 SIMD.
- Benchmark with relative gates (runner-speed-proof) wired into CI.

**Non-Goals:**
- Multithreading (blocked by Pages' missing cross-origin isolation headers).
- Pan-blit (translate old frame, render only the exposed strip) — complexity; progressive covers responsiveness.
- Adaptive iteration counts by zoom depth (changes visual results mid-interaction).
- Deep-zoom precision (soft-double), tile-based progressive refinement.

## Decisions

- **Scale parameter on `render(variant, cx, cy, height, max_iter, scale)` writing the first (W/s)·(H/s) buffer entries**, instead of a second export. One code path; JS swaps the canvas internal size to (W/s, H/s) during preview — the browser upscales via CSS (soft/blurry is acceptable for a live preview) — then restores 960×640 for the settle render. Alternative considered: separate `render_scaled` export — rejected, duplicate entry point for no gain.
- **Settle timer 150 ms** in JS after the last wheel/pointer event → one full render. Alternative: render full-res on rAF idle — rejected, rAF keeps firing during continuous input.
- **Dispatch hoisting by monomorphizing per-variant inner loops** (render selects one of 5 loop functions once). Expected small win (the match is branch-predictable), but it is the prerequisite for a clean SIMD loop; near-zero risk.
- **f64x2 SIMD (`core::arch::wasm32`, stable) processing 2 pixels per iteration** — escape iteration is embarrassingly parallel across pixels. Kept ONLY if the benchmark shows ≥1.3× over the scalar hoisted loop; otherwise deleted (ponytail: don't ship SIMD that doesn't pay).
- **Benchmark gate is relative, not absolute**: scale-2 speedup ≥ 2.5× is robust across runner speeds; absolute fence 250 ms only catches pathological regressions. Median of 5 runs per measurement. Node ≈ browser for pure wasm compute, so Node timing is representative.
- **`wasm-opt -O3`** via Ubuntu's `binaryen` apt package — no third-party action, no npm.

## Risks / Trade-offs

- [CI runner variance makes absolute timing flaky] → gate is relative; fence is 2.5× the local median with headroom.
- [Low-res preview artifacts (aliasing) during fast interaction] → transient by design (~150 ms), replaced by full-res on settle.
- [SIMD path doubles the loop implementations] → acceptance gate ≥1.3× or it gets deleted; the scalar path stays the fallback.

## Migration Plan

Single commit on `main`; CI gates the deploy. Rollback = revert commit, previous deployment untouched (Pages keeps last good artifact only if build fails before deploy — a deployed bad frame would need a revert push).

## Open Questions

None.
