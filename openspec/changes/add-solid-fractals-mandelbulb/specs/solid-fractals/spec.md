## Purpose

True 3D fractals (Mandelbulb, Mandelbox) computed as signed-distance fields in Rust/WASM and rendered as orbitable three.js isosurfaces.

## ADDED Requirements

### Requirement: Signed-distance field computation
The renderer SHALL compute a per-voxel distance estimate for the selected solid fractal variant — Mandelbulb at powers 2, 3, 4, 6, 8 and Mandelbox (scale 3) — into a readable field buffer, filling one requested Z-slab per call so the UI can yield between slabs. Points that never escape SHALL have negative field values; the isosurface at zero SHALL lie on the fractal boundary.

#### Scenario: Field separates inside from outside
- **WHEN** the Mandelbulb field is computed over its bounding window
- **THEN** voxels near the origin have negative values, voxels far outside have positive values, and a sign change exists between them

#### Scenario: Slabs compose to the whole grid
- **WHEN** the grid is computed in two halves via separate slab calls
- **THEN** the result equals a single-call computation over the full range

### Requirement: Isosurface rendering
The 3D viewer SHALL extract a triangulated isosurface (marching cubes) from the field at value zero, with normals derived from the field gradient, rendered as a lit solid mesh.

#### Scenario: Bulb is visible as a solid
- **WHEN** the Mandelbulb (power 8) is rendered at standard detail
- **THEN** a connected, lit, non-empty mesh is displayed inside the bounding window

### Requirement: Solid-fractal controls
The viewer SHALL offer the solid fractal variants, at least three detail (grid resolution) levels, an auto-rotate toggle, and orbit/zoom/pan navigation. Re-computation SHALL show progress and keep the page responsive.

#### Scenario: Switching variant recomputes with progress
- **WHEN** the user selects Mandelbox
- **THEN** a progress indication is shown while slabs compute and the isosurface updates when complete

### Requirement: Navigation between viewers
The solid-fractal viewer SHALL link to the 2D and terrain viewers, which link back.

#### Scenario: User switches viewers
- **WHEN** the user follows the link from the terrain page
- **THEN** the solid-fractal page loads from the same deployment
