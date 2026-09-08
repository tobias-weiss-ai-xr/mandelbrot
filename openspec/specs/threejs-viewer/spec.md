# threejs-viewer Specification

## Purpose
Orbitable 3D heightfield exploration of the Mandelbrot-set variants, rendered with three.js from the WASM compute engine's output.

## Requirements

### Requirement: 3D heightfield rendering
The 3D viewer SHALL render the selected variant as a heightfield where each pixel's height derives from its escape iteration count (interior pixels flat) and each vertex carries the color from the 2D render palette. The heightfield SHALL be lit and rendered as a solid surface.

#### Scenario: Terrain reflects the fractal
- **WHEN** the Mandelbrot variant is displayed in 3D
- **THEN** the region outside the set forms elevated terrain that falls off toward the set boundary, and interior pixels form a flat region

#### Scenario: Switching variant rebuilds the terrain
- **WHEN** the user selects "Burning Ship" in the 3D viewer
- **THEN** the heightfield and vertex colors update to that variant's default view

### Requirement: 3D camera navigation
The viewer SHALL support orbit (drag), zoom (wheel), and pan via OrbitControls, with inertial damping, plus an auto-rotate toggle.

#### Scenario: Orbiting costs no re-render
- **WHEN** the user drags to orbit the camera
- **THEN** the view rotates smoothly without recomputing the fractal

### Requirement: Detail control
The viewer SHALL offer at least two detail levels (grid resolutions) that re-render the fractal at the corresponding downscale factor and rebuild the heightfield.

#### Scenario: Lower detail rebuilds faster
- **WHEN** the user switches from the highest to a lower detail level
- **THEN** the terrain rebuilds from a proportionally smaller grid

### Requirement: Navigation between viewers
The 2D viewer SHALL link to the 3D viewer and vice versa.

#### Scenario: User switches views
- **WHEN** the user follows the link from the 2D page
- **THEN** the 3D terrain page loads with the WASM module from the same deployment
