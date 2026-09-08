## 1. WASM field computation

- [x] 1.1 Static `FIELD` buffer (f32, capped grid), `field_ptr()` export, slab-wise `bulb_field(variant, grid, iters, z0, z1)` with Mandelbulb powers {2,3,4,6,8} (mul-chain rⁿ, φ recurrence) and Mandelbox s3; DE sign convention per design
- [x] 1.2 Tests: inside-negative/outside-positive, slab composition equals full-range call, box variant sane

## 2. Viewer page

- [x] 2.1 Vendor `web/vendor/objects/MarchingCubes.js`; confirm field layout/API from source
- [x] 2.2 `web/bulb.html`: variant select (bulb p8/p2/p3/p4/p6, mandelbox), detail select, auto-rotate, progress %, chunked compute loop, isosurface build, cross-links to 2D/terrain
- [x] 2.3 Links from `index.html` and `terrain.html` to `bulb.html`

## 3. Verification & deployment

- [x] 3.1 Node smoke: slab composition, field sign structure, MarchingCubes headless build (non-empty geometry, finite normals); measure per-grid timings → set shipped grid levels
- [x] 3.2 `openspec validate --all`, commit, push, CI green, live pages 200 (bulb.html, MarchingCubes asset)
