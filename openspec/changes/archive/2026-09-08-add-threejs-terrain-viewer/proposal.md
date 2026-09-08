## Why

The 2D canvas viewer shows escape counts only as color. A three.js 3D view turns the same escape-count data into an orbitable heightfield terrain — the natural three.js rendering of Mandelbrot-set variants — while keeping the mandated Rust/WASM compute engine (no GLSL fractal duplication).

## What Changes

- WASM renderer additionally exposes per-pixel iteration counts through an `iters_ptr()` buffer, filled during the existing `render` call at zero extra pass cost.
- New `terrain.html` page: three.js scene builds an orbitable 3D heightfield from the iteration buffer, vertex-colored from the existing render palette; OrbitControls rotate/zoom/pan, auto-rotate toggle, variant and detail selectors.
- three.js is vendored as static ES modules (no CDN dependency, no npm build).
- The 2D page links to the 3D view; the workflow ships the whole `web/` directory.

## Capabilities

### New Capabilities
- `threejs-viewer`: 3D heightfield exploration of the fractal variants with three.js.

### Modified Capabilities
- `fractal-rendering`: adds a requirement for a readable per-pixel iteration-count buffer.

## Impact

- `src/lib.rs`: static `ITERS` buffer, `iters_ptr` export, fill in render loop; one more test.
- `web/terrain.html` (new), `web/vendor/` (three.module.min.js, OrbitControls.js), link in `web/index.html`.
- `.github/workflows/deploy.yml`: copy the full `web/` directory instead of a single file.
