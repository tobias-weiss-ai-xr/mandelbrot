# viewer-ui Specification

## Purpose
Browser UI that loads the WASM renderer and lets the user explore the fractal variants on a canvas.

## Requirements

### Requirement: Canvas display
The UI SHALL display the rendered fractal on a canvas element, sized to the fixed internal resolution of the renderer and scaled to fit the viewport width.

#### Scenario: Page loads with a rendered image
- **WHEN** the page is opened in a browser
- **THEN** a fractal image is visible without any user interaction

### Requirement: Variant and iteration controls
The UI SHALL offer a control to choose any of the supported variants and a control to change the iteration limit. Selecting a variant SHALL switch to that variant's default viewport and re-render.

#### Scenario: Switching variant re-renders
- **WHEN** the user selects "Burning Ship" after viewing "Mandelbrot"
- **THEN** the canvas shows the Burning Ship at its default viewport

#### Scenario: Raising iterations adds detail
- **WHEN** the user increases the iteration control
- **THEN** the fractal re-renders with the new limit without changing the viewport

### Requirement: Zoom and pan
The UI SHALL zoom with the mouse wheel, anchored at the cursor position, and pan by dragging with the pointer. Each interaction SHALL trigger a re-render of the new viewport.

#### Scenario: Wheel zoom is cursor-anchored
- **WHEN** the user scrolls the wheel up while the cursor is over a feature
- **THEN** the viewport height shrinks and the complex-plane point under the cursor stays under the cursor

#### Scenario: Drag pans the view
- **WHEN** the user drags the canvas by 100 screen pixels
- **THEN** the view shifts by the corresponding complex-plane distance and re-renders
