## Context

2D viewer ships a static RGBA buffer from Rust/WASM; three.js enters as a rendering layer, not a compute layer. GitHub Pages + zero-build constraint rules out npm bundling. The scale param (added in tune-renderer-performance) already renders reduced grids cheaply — the 3D page reuses it directly.

## Goals / Non-Goals

**Goals:**
- Same WASM module powers both viewers; one `render` call feeds colors and heights.
- Static vendored three.js, importmap, no build step.
- Geometry reused across re-renders at a fixed detail level.

**Non-Goals:**
- GLSL shader fractal rendering (bypasses the mandated Rust compute).
- Fractal zoom inside the 3D view (camera orbit is the interaction; fractal viewport stays at per-variant defaults).
- True 3D fractals (Mandelbulb etc.), animation of parameter sweeps, workers.

## Decisions

- **`iters_ptr()` export over a render-mode flag**: render already has `n` per pixel; writing it to a parallel static `ITERS: [u32; W*H]` costs one store and keeps `render`'s signature stable. Colors and counts are then always from the same invocation — no staleness.
- **Height = sqrt(n/max_iter) · H_MAX, interior flat at 0.** Square root mimics the palette's perceived falloff; linear looks spiky at low n. H_MAX ≈ 2.5 world units on a 16-unit plane.
- **Vertex colors straight from the RGBA buffer** (skip alpha) — the terrain looks exactly like the 2D fractal for free; no second palette implementation.
- **PlaneGeometry(w−1, h−1, w−1, h−1)** — vertices map 1:1 to pixels; Z displaced per vertex; `computeVertexNormals()` per rebuild (once per variant/detail change, not per frame). Lambert material + directional light for terrain shading.
- **Detail levels scale 4 (default) / 2 / 8** → 38k / 153k / 9.6k vertices. Scale 2 is plenty at 153k verts; 4 is the snappy default.
- **Vendored three@0.160.0** (`three.module.min.js` + `OrbitControls.js`) + importmap mapping `three` → `./vendor/three.module.min.js`. Unpkg URLs break builds-by-network; vendored files are committed and served by Pages like any other asset.

## Risks / Trade-offs

- [computeVertexNormals cost at scale 2 (153k verts) on rebuild] → one-shot per change (~tens of ms), acceptable; default is 38k.
- [Vendored library goes stale] → pinned version, upgrade is a file swap; no security surface beyond static asset.
- [ITERS buffer adds 2.4 MB to WASM linear memory] → memory is already 2.4 MB for colors; total well under defaults.

## Migration Plan

Single commit; new page is additive. Rollback = revert.

## Open Questions

None.
