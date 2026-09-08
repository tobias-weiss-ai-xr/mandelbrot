## Context

Empty repo, mandated stack: Rust compiled to WebAssembly, published on GitHub Pages. Toolchain on dev machine: no local Rust install needed for CI (build happens in Actions), but local `cargo` is absent — verification runs in CI or via a temporary toolchain.

## Goals / Non-Goals

**Goals:**
- Zero-JS-framework, zero-npm frontend; plain `WebAssembly.instantiateStreaming`.
- Rust exports raw functions and memory; no wasm-bindgen, no glue code generation.
- One workflow file does build + deploy.

**Non-Goals:**
- Web Workers / threaded rendering, progressive tiles, deep-zoom arbitrary precision.
- Adjustable Julia parameter (mouse-driven c), animation, saving screenshots.
- Responsive/devicePixelRatio-aware canvas sizing (fixed internal resolution, CSS-scaled).

## Decisions

- **Plain `wasm32-unknown-unknown` with `#[no_mangle] pub extern "C"` exports instead of wasm-bindgen/wasm-pack.** The interface is 5 scalars + a fixed static buffer — strings, JS structs, or closures are never passed. wasm-bindgen would add a dependency, a CLI step in CI, and generated glue for zero benefit here.
- **Fixed static pixel buffer (`static mut BUF: [u32; W*H]`) at 960×640.** No allocation API, no memory-growth edge cases in JS; `addr_of_mut!` avoids `static_mut_refs`. Rendering is single-threaded, one full-buffer pass per frame — ~600k pixels of f64 math re-render in well under a second at 256 iters; good enough for interactive zoom.
- **Little-endian u32 RGBA packed in Rust** (`r | g<<8 | b<<16 | 0xFF<<24`), viewed in JS as `Uint8ClampedArray` → `ImageData` → `putImageData`. No per-pixel JS loop.
- **Variant = `u32` discriminant matched in Rust; per-variant default viewports live in JS.** Keeps the ABI numeric-only.
- **Smooth coloring** `mu = n + 1 − log2(ln|z|²/2)` driving a cosine palette; interior black. Cheap, continuous across zooms.
- **CI: `dtolnay/rust-toolchain@stable` + `cargo build --release --target wasm32-unknown-unknown`, copy `web/index.html` + `.wasm` into `site/`, `upload-pages-artifact` + `deploy-pages`.** No wasm-pack, no npm install in CI.

## Risks / Trade-offs

- [Single-threaded render jank on deep zooms at 512+ iterations] → acceptable for v1; iteration control lets users trade quality for speed. Workers + SharedArrayBuffer if it ever matters.
- [`static mut` unsound in principle with concurrent calls] → JS never calls `render` concurrently (rAF-debounced single call path).
- [f64 precision floor at extreme zoom (~1e-13 view height)] → out of spec scope; deep-zoom would need soft-double, a deliberate future change.

## Migration Plan

Initial deployment: push to `main`, set repo Pages source to "GitHub Actions". Rollback = redeploy previous commit.

## Open Questions

None.
