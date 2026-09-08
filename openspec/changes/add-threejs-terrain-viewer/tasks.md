## 1. WASM iteration buffer

- [x] 1.1 Add static `ITERS` buffer, fill during `render`, export `iters_ptr()`; test: interior == max_iter, all ≤ max_iter, scale-1/scale-2 agreement at shared pixels

## 2. three.js assets

- [x] 2.1 Vendor three@0.160.0: `web/vendor/three.module.min.js` + `web/vendor/controls/OrbitControls.js` (pinned, committed)

## 3. 3D viewer page

- [x] 3.1 `web/terrain.html`: heightfield scene (geometry reuse, vertex colors from RGBA buffer, z from sqrt(n/max)), variant select, detail select (scale 2/4/8), auto-rotate toggle, OrbitControls with damping
- [x] 3.2 Cross-links between `index.html` and `terrain.html`

## 4. Deployment & verification

- [x] 4.1 Workflow: `cp -r web/. site/`
- [x] 4.2 Node smoke: iters buffer contract + geometry/z-height math against three.js (module import, no WebGL needed)
- [x] 4.3 `openspec validate --all`, commit, push, CI green, live pages 200 (terrain.html, vendor assets)
