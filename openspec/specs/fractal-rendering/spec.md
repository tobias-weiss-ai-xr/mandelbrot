# fractal-rendering Specification

## Purpose
Escape-time rendering of Mandelbrot-set variants into a pixel buffer, computed in Rust and exposed via WebAssembly.

## Requirements

### Requirement: Render supported fractal variants
The renderer SHALL support the variants Mandelbrot (z²+c), Julia (z²+c with fixed c = −0.7+0.27015i), Multibrot-3 (z³+c), Burning Ship ((|Re z|, |Im z|)²+c), and Tricorn (conj(z)²+c), selectable by a variant identifier.

#### Scenario: Mandelbrot interior is dark
- **WHEN** the Mandelbrot variant is rendered with the default view (center −0.6+0i, height 2.7)
- **THEN** pixels inside the set (c = 0 maps to interior) are near-black and pixels at the cardoid boundary show non-black colors

#### Scenario: Burning Ship shows its body
- **WHEN** the Burning Ship variant is rendered with center −1.75−0.03i, height 0.15
- **THEN** the buffer contains a non-trivial mix of interior (dark) and exterior (colored) pixels

### Requirement: Escape-time pixel value
For each pixel the renderer SHALL iterate the variant's recurrence from the pixel's complex seed until |z|² > 4 or the iteration limit is reached. Escaped pixels SHALL be colored by a smooth function of the escape iteration count; non-escaped pixels SHALL be colored black. All pixels SHALL be fully opaque (alpha = 255).

#### Scenario: Exterior pixels vary with iteration count
- **WHEN** two exterior pixels have different escape iteration counts
- **THEN** their colors differ according to the smooth coloring function

#### Scenario: Iteration limit changes interior size
- **WHEN** the same view of the Mandelbrot is rendered with max_iter = 64 and max_iter = 512
- **THEN** the higher limit produces an equal or smaller set of interior (black) pixels (slowly escaping boundary points get revealed as exterior)

### Requirement: View is parameterized
The renderer SHALL accept a viewport (center x, center y, height in complex-plane units, iteration limit) and map the canvas height to that height, preserving square aspect (same units per pixel in x and y).

#### Scenario: Zooming in increases detail
- **WHEN** the same viewport is rendered twice, second time with height reduced 10× at the same center
- **THEN** both renders fill the buffer, and the second shows different structure at the zoomed location

### Requirement: Stable buffer contract
The renderer SHALL expose a pointer to a fixed-size RGBA pixel buffer (canvas width × height × 4 bytes, little-endian) and a render function; after render returns, the buffer SHALL contain the image for the requested view.

#### Scenario: Buffer is readable after render
- **WHEN** a render call completes
- **THEN** reading the exported buffer pointer yields width×height u32 values, each with the high byte = 255

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

### Requirement: Iteration-count buffer
The renderer SHALL make per-pixel escape iteration counts readable through an `iters_ptr()` export. After a render with scale s, the first (W/s)·(H/s) entries hold each pixel's step count (equal to the iteration limit for interior pixels), matching the color buffer layout.

#### Scenario: Counts match interior/extreme classification
- **WHEN** the Mandelbrot default view is rendered at 256 iterations
- **THEN** interior pixels read exactly 256, all entries are ≤ 256, and the count at a shared pixel is identical between scale-1 and scale-2 renders of the same view
