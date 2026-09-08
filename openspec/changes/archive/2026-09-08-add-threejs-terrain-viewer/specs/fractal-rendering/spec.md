## Purpose

Escape-time rendering of Mandelbrot-set variants into a pixel buffer, computed in Rust and exposed via WebAssembly.

## ADDED Requirements

### Requirement: Iteration-count buffer
The renderer SHALL make per-pixel escape iteration counts readable through an `iters_ptr()` export. After a render with scale s, the first (W/s)·(H/s) entries hold each pixel's step count (equal to the iteration limit for interior pixels), matching the color buffer layout.

#### Scenario: Counts match interior/extreme classification
- **WHEN** the Mandelbrot default view is rendered at 256 iterations
- **THEN** interior pixels read exactly 256, all entries are ≤ 256, and the count at a shared pixel is identical between scale-1 and scale-2 renders of the same view
