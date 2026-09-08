## Why

The repository is empty. The goal is an interactive visualization of different Mandelbrot-set variants (Mandelbrot, Julia, Multibrot-3, Burning Ship, Tricorn) that runs in the browser and is published automatically via GitHub Pages. Rust is the mandated implementation language, so the fractal iteration math compiles to WebAssembly.

## What Changes

- Add a Rust crate (`mandelbrot`, `cdylib`) that computes fractal pixel buffers into WASM memory for 5 variants with smooth escape-time coloring.
- Add a single-page frontend (`web/index.html`, no framework, no npm) that loads the WASM module, renders to a canvas, and supports variant selection, iteration control, wheel zoom, and drag pan.
- Add a GitHub Actions workflow that builds the WASM on push to `main` and deploys the site to GitHub Pages.

## Capabilities

### New Capabilities
- `fractal-rendering`: Escape-time rendering of Mandelbrot-set variants into a pixel buffer (WASM exports).
- `viewer-ui`: Browser canvas UI with variant selection, iteration control, zoom, and pan.
- `pages-deployment`: CI build and GitHub Pages publication of the viewer.

### Modified Capabilities

## Impact

- New files: `Cargo.toml`, `src/lib.rs`, `web/index.html`, `.github/workflows/deploy.yml`.
- No dependencies beyond the Rust toolchain (`wasm32-unknown-unknown` target); frontend is dependency-free.
- Requires Pages source set to "GitHub Actions" in repo settings.
