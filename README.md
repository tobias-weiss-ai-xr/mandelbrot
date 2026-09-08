# Mandelbrot Set Variants

Interactive viewer for Mandelbrot, Julia, Multibrot-3, Burning Ship and Tricorn sets.
Fractal math in Rust compiled to WebAssembly (no wasm-bindgen, no npm), UI is a single
static HTML page, deployed to GitHub Pages by `.github/workflows/deploy.yml`.

Scroll to zoom (cursor-anchored), drag to pan, pick a variant, tune iterations.

Requires the repo's Pages source set to **GitHub Actions** (Settings → Pages).
