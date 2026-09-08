## Context

The stack mandate (Rust computes, JS presents) rules out the classic GPU raymarched Mandelbulb. A CPU voxel field + marching cubes produces a real mesh that orbits at full frame rate; surface detail is capped by grid resolution. WASM sin/cos/acos/atan2 are the cost drivers (~5 transcendental calls per iteration), so grid sizes must be chosen from measurements, not hope.

## Goals / Non-Goals

**Goals:**
- Mandelbulb (5 powers) + Mandelbox as orbitable solid meshes.
- Responsive chunked computation with progress.
- Zero new dependencies beyond the vendored MarchingCubes addon.

**Non-Goals:**
- GPU raymarching (violates stack mandate), 4D Quaternion Juliabulbs, animation of power sweeps, workers (transferable-slab parallelism possible later; sequential measured sufficient).
- Sub-voxel detail (adaptive octree refinement).

## Decisions

- **DE (distance estimator) as the field, isosurface at 0** — DE is continuous, so marching cubes interpolates a smooth boundary with gradient normals; raw escape counts would give a stair-step surface. Interior (never escaped) = constant negative sentinel (-0.05); exterior DE = 0.5·ln(r)·r/dr (bulb) or |z|/dr (box).
- **Grid sizes 32³/48³/64³/96³** (Coarse/Standard/Fine/Ultra). Measured (this machine, wasm-opt): bulb p8 32³ 17 ms → 96³ 340 ms; Mandelbox s3 96³ 44 ms. Early-escape bailout keeps even the max grid interactive; all four levels ship.
- **No `powf`**: rⁿ for n ∈ {2,3,4,6,8} built from r and r² by multiplication; φ multiplied via complex-recurrence (≤7 muls) instead of cos/sin of the scaled angle. Per iteration: atan2, acos, one sin/cos pair, one sqrt. Slab chunking (8 planes per call) yields to the event loop; progress = z1/grid.
- **Vendored `MarchingCubes.js` with field injected directly** (`effect.field.set(...)`, `effect.isolation = 0`, one `update()` per build) — the addon owns triangulation + gradient normals; no second MC implementation.
- **Variant bounds in Rust**: bulb window [−1.2, 1.2]³, Mandelbox s3 [−1.8, 1.8]³; field in world units, mesh scaled to ~14 world units in the scene.

## Risks / Trade-offs

- [Trig-heavy bulb iteration limits grid resolution] → measured grid caps; slab chunking hides the wait; workers (transferable slabs) are the upgrade path.
- [Interior sentinel plateau distorts normals at the boundary] → only affects the inner side of the surface; visually negligible with gradient normals.
- [MarchingCubes default maxPolyCount too low] → constructor takes an explicit cap (≥ 400k triangles).

## Migration Plan

Additive page; revert = rollback.

## Open Questions

None (grid caps settled by measurement during apply).
