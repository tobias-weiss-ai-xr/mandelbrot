## Purpose

Escape-time rendering of Mandelbrot-set variants into a pixel buffer, computed in Rust and exposed via WebAssembly.

## ADDED Requirements

### Requirement: Reduced-resolution rendering
The renderer SHALL support rendering at an integer downscale factor (at least scale 2): the rendered image has width/scale × height/scale pixels, occupies the first (width/scale)·(height/scale) entries of the buffer, and covers the same complex-plane viewport as a scale-1 render. All pixels SHALL be opaque.

#### Scenario: Low-res render is compact and opaque
- **WHEN** the Mandelbrot default view is rendered with scale 2
- **THEN** the first (W/2)·(H/2) buffer entries are opaque RGBA pixels and the viewport content corresponds to the scale-1 render

#### Scenario: Low-res render is much faster
- **WHEN** the same view and iteration limit are rendered at scale 2 and scale 1
- **THEN** the scale-2 render takes less than 1/2.5 of the scale-1 time

### Requirement: Render performance budget
A full-resolution render of the Mandelbrot default view at 256 iterations SHALL complete in ≤ 100 ms on the CI benchmark runner as a target, with a hard CI regression fence at 250 ms; per-iteration variant dispatch SHALL be hoisted out of the iteration loop.

#### Scenario: Default view meets the fence
- **WHEN** the CI benchmark renders the Mandelbrot default view at 256 iterations
- **THEN** the measured median time is below the 250 ms fence and is recorded for trend comparison
