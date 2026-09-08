## 1. Rust renderer crate

- [x] 1.1 Create `Cargo.toml` (`cdylib`, edition 2021, release profile with LTO) and `src/lib.rs` with the static pixel buffer, `buf_ptr` and `render` exports
- [x] 1.2 Implement the 5 variant recurrences (Mandelbrot, Julia, Multibrot-3, Burning Ship, Tricorn), escape loop, smooth coloring, view→pixel mapping
- [x] 1.3 Add `#[test]`s for spec scenarios: interior dark at c=0, exterior colors vary with escape count, higher max_iter never shrinks interior, buffer alpha = 255; run `cargo test`

## 2. Frontend

- [x] 2.1 Create `web/index.html`: canvas, variant dropdown, iteration slider, WASM streaming-instantiation + `putImageData` render path
- [x] 2.2 Wire wheel zoom (cursor-anchored) and drag pan with rAF-debounced re-render; per-variant default viewports

## 3. Deployment

- [x] 3.1 Create `.github/workflows/deploy.yml`: wasm build → `site/` assembly → upload-pages-artifact → deploy-pages, on push to `main` + `workflow_dispatch`
- [x] 3.2 Add minimal `README.md` (what it is, Pages settings note)

## 4. Verification

- [x] 4.1 Build wasm artifact; smoke-test exports in Node (render each variant, check buffer alpha and non-uniformity)
- [x] 4.2 `openspec validate --all` passes; commit and push; confirm workflow goes green and site renders
