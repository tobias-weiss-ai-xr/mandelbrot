## Purpose

Build the WASM viewer in CI and publish it to GitHub Pages on every push to main.

## ADDED Requirements

### Requirement: Optimized wasm artifact
CI SHALL run a size/speed optimization pass (binaryen `wasm-opt`) over the built wasm module before uploading the Pages artifact, and the benchmark step SHALL gate the deployment on the performance thresholds defined in fractal-rendering.

#### Scenario: Deployed artifact is optimized
- **WHEN** the workflow builds the site
- **THEN** the uploaded wasm is the wasm-opt-processed module

#### Scenario: Benchmark gate blocks bad builds
- **WHEN** the benchmark exceeds the regression fence or the low-res speedup falls below 2.5×
- **THEN** the workflow fails before deploying
