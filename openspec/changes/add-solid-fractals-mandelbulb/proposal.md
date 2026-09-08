## Why

The terrain viewer extrudes 2D escape counts; it cannot show fractals that only exist in 3D. Mandelbulb (z ← zⁿ + c in spherical coordinates) and Mandelbox (box/sphere folds) are true volumetric fractals requiring per-voxel signed-distance fields and isosurface extraction.

## What Changes

- Rust computes a signed-distance field (DE, negative inside) over a voxel grid for Mandelbulb (powers 2/3/4/6/8) and Mandelbox (scale 3), chunked by Z-slabs so the page stays responsive with a progress indicator.
- New `bulb.html` page: three.js `MarchingCubes` isosurface of the DE field, orbitable, auto-rotate, variant/detail selectors, cross-links with the other viewers.
- three.js MarchingCubes addon vendored alongside the existing assets.

## Capabilities

### New Capabilities
- `solid-fractals`: true 3D fractals (Mandelbulb, Mandelbox) as signed-distance fields in WASM with a three.js isosurface viewer.

### Modified Capabilities

## Impact

- `src/lib.rs`: static `FIELD` buffer, `field_ptr()` + slab-wise `bulb_field()` exports; field tests.
- `web/bulb.html` (new), `web/vendor/objects/MarchingCubes.js`, link in `web/index.html` + `web/terrain.html`.
- No workflow change (`cp -r web/.` already ships vendor assets).
- Compute is CPU-bound: grid sizes are capped so the standard level stays interactive.
