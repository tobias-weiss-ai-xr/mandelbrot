# Benchmark notes — tune-renderer-performance

Node 22, this machine, median of 5 (scripts/bench.mjs), full-res = 960×640.

## Baseline (commit 53d5c4d, per-iteration variant dispatch)

| Variant       | full ms | low ms | speedup |
|---------------|---------|--------|---------|
| Mandelbrot    | 140.0   | –      | –       |
| Julia         | 73.0    | –      | –       |
| Multibrot 3   | 103.0   | –      | –       |
| Burning Ship  | 66.0    | –      | –       |
| Tricorn       | 70.0    | –      | –       |

(Initial smoke timings; no scale support, no binaryen.)

## After: dispatch hoisting + scale support (scalar)

| Variant       | full ms | low ms | speedup |
|---------------|---------|--------|---------|
| Mandelbrot    | 93.1    | 23.2   | 4.01×   |
| Julia         | 77.2    | 19.6   | 3.94×   |
| Multibrot 3   | 112.6   | 28.0   | 4.02×   |
| Burning Ship  | 77.3    | 19.9   | 3.89×   |
| Tricorn       | 79.9    | 20.0   | 3.99×   |

## After: wasm-opt -O3

| Variant       | full ms | low ms | speedup | size |
|---------------|---------|--------|---------|------|
| Mandelbrot    | 102.3   | 25.1   | 4.07×   | 10682 B (from 12348 B) |
| Julia         | 81.3    | 20.9   | 3.89×   | |
| Multibrot 3   | 120.7   | 27.6   | 4.37×   | |
| Burning Ship  | 81.2    | 19.2   | 4.24×   | |
| Tricorn       | 78.8    | 21.0   | 3.75×   | |

wasm-opt: size −13 %, runtime neutral on this machine (LLVM already
vectorizes/schedules the scalar loop well; −O3 wins are mostly size here).

## f64x2 SIMD attempt — DELETED per acceptance gate

Implemented (2 adjacent pixels/iteration, frozen-escape masks). After fixing a
`v128_bitselect` argument-order bug, it was correct (100 % scale1/scale2
consistency) but SLOWER than scalar: Mandelbrot 105.8 ms vs 93.1 ms. The
freeze/Select scaffolding and lane extraction outweigh the 2-lane gain against
LLVM's lean scalar code. Deleted per the ≥1.3× acceptance rule; the scalar
hoisted loop ships.

## Gates (CI, scripts/bench.mjs)

- full-res Mandelbrot fence: 250 ms ✓ (measured ~80–120 ms)
- preview speedup ≥ 2.5× ✓ (measured ~4×)
- scale1/scale2 pixel consistency ≥ 99.9 % ✓ (measured 100 %)

## Verdict

The user-visible win is interactivity: wheel/drag now draws ~20–30 ms preview
frames instead of queueing 80–120 ms full renders; one full render lands 150 ms
after the last input event.
