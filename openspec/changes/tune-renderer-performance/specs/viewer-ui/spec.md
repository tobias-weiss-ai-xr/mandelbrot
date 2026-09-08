## Purpose

Browser UI that loads the WASM renderer and lets the user explore the fractal variants on a canvas.

## ADDED Requirements

### Requirement: Progressive interaction rendering
During wheel zoom or pointer drag the UI SHALL display reduced-resolution (at least 1/4 pixel count) frames, and SHALL schedule exactly one full-resolution render after interaction has settled (~150 ms without input). No full-resolution render SHALL be scheduled per interaction event.

#### Scenario: Wheel spam stays responsive
- **WHEN** the user scrolls the wheel rapidly several times in a row
- **THEN** only low-res frames are drawn while scrolling and a single full-res frame appears after the last event settles

#### Scenario: Settled view is full resolution
- **WHEN** interaction stops and the settle timer elapses
- **THEN** the canvas shows the full-resolution render of the current viewport

### Requirement: Render-time readout
The UI SHALL display the duration of the last full-resolution render in milliseconds.

#### Scenario: Readout updates after full render
- **WHEN** a full-resolution render completes
- **THEN** the displayed value is the measured duration of that render in ms
