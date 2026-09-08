# pages-deployment Specification

## Purpose
Build the WASM viewer in CI and publish it to GitHub Pages on every push to main.

## Requirements

### Requirement: Build and deploy on push
On every push to `main` (and manual trigger), CI SHALL compile the Rust crate for `wasm32-unknown-unknown`, assemble the site (frontend + built `.wasm`), and deploy it as a GitHub Pages artifact.

#### Scenario: Push to main publishes the site
- **WHEN** a commit is pushed to `main`
- **THEN** the workflow builds the WASM, uploads the site artifact, and deploys it to GitHub Pages

#### Scenario: Broken build fails before deploy
- **WHEN** the Rust crate fails to compile
- **THEN** the workflow fails and the previous Pages deployment remains untouched

### Requirement: Site is self-contained
The deployed site SHALL consist only of static files (HTML + `.wasm`) served by GitHub Pages, loadable without a build step at request time.

#### Scenario: WASM loads with correct MIME type
- **WHEN** the page fetches the `.wasm` file from GitHub Pages
- **THEN** streaming instantiation succeeds (Pages serves `.wasm` as `application/wasm`)

### Requirement: Optimized wasm artifact
CI SHALL run a size/speed optimization pass (binaryen `wasm-opt`) over the built wasm module before uploading the Pages artifact, and the benchmark step SHALL gate the deployment on the performance thresholds defined in fractal-rendering.

#### Scenario: Deployed artifact is optimized
- **WHEN** the workflow builds the site
- **THEN** the uploaded wasm is the wasm-opt-processed module

#### Scenario: Benchmark gate blocks bad builds
- **WHEN** the benchmark exceeds the regression fence or the low-res speedup falls below 2.5×
- **THEN** the workflow fails before deploying
