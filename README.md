# Powder Toy SLT

A falling-sand physics simulator inspired by [The Powder Toy](https://powdertoy.co.uk/), built entirely in **Rust** with **[SuperLightTUI](https://github.com/subinium/SuperLightTUI)** for terminal rendering.

No GPU. No window manager. No SDL. Just your terminal.

![demo](demo.gif)

```
cargo run --release
```

## Why This Exists

The original Powder Toy is a C++/SDL2 desktop application with 20+ years of history. This project asks a simple question: **can a full particle physics sandbox run in a terminal?**

The answer is yes — powered by two things:

- **Rust** for zero-cost abstractions, safe concurrency, and raw simulation speed
- **SuperLightTUI** for immediate-mode TUI rendering with half-block pixels, mouse input, and 30fps double-buffered output

The result is a 5,000-line Rust binary that simulates 151 elements with pressure, heat, airflow, and electricity — all inside your terminal emulator.

## Features

### Physics Engine

| System | Description |
|---|---|
| Gravity | Normal, Zero, Inverted modes |
| Pressure | Per-cell field with diffusion, wall blocking, explosion shockwaves |
| Heat | Thermal conduction, ambient temperature, phase transitions |
| Airflow | Velocity fields (vx/vy), pressure-gradient driven, Fan injection |
| Electricity | Wire/Spark propagation, Battery, PSCN/NSCN directional, SWCH toggle, DLAY delay, INST instant |
| Phase Changes | Water/Ice/Steam, Stone/Lava, Methane/Fire, Hydrogen/Plasma with hysteresis |
| Reactions | 50+ rules: combustion, acid, nuclear fission, explosions, sensor triggers |
| Life | Conway's Game of Life (B3/S23) |

### 151 Elements across 10 Categories

| Category | Count | Examples |
|---|---|---|
| Powder | 21 | Sand, Dust, Coal, Gunpowder, Snow, Salt, Sawdust, Thermite, Quartz |
| Liquid | 17 | Water, Oil, Lava, Acid, Mercury, Alcohol, Cryo, Deuterium, Glow |
| Gas | 13 | Fire, Steam, Smoke, Hydrogen, Plasma, Oxygen, Methane, Fog |
| Solid | 27 | Stone, Metal, Wood, Ice, Glass, Brick, Diamond, Gold, Iron, Tungsten, Silicon |
| Electronics | 7 | Wire, Battery, PSCN, NSCN, SWCH, ARAY, Insulator |
| Energy | 7 | Photon, Neutron, Uranium, Bomb, Spark, Proton, Electron |
| Special | 14 | Wall, Cloner, Void, Fan, Grav, Black Hole, White Hole, Singularity |
| Sensor | 8 | DTEC, TSNS, PSNS, PCLN, PBCN, DRAY, CRAY, HSWC |
| Transport | 5 | Pipe, Portal In/Out, WiFi A/B |
| Radioactive | 4 | Plutonium, Anti-matter, Positron, Electron |

### Terminal UI

- **Half-block rendering** (`▀`) for 2x vertical pixel density
- **5 view modes**: Normal, Pressure, Heat, Electric, Airflow
- **Mouse drawing** with Circle/Square/Line brush shapes
- **Element palette** with category navigation, search (`/`), and click-to-select
- **Zoom/Pan** (1x-4x) with viewport controls
- **Stamp system** — copy (`c`) and paste (`x`) rectangular regions
- **Save/Load** across 5 slots with undo/redo (64-frame stack)
- **Live telemetry**: FPS, TPS, particle count, pressure/heat/velocity stats, element breakdown, sparkline history
- **Grid overlay**, screenshot export, element info display

## Controls

| Key | Action |
|---|---|
| `Click` | Draw with selected element |
| `1-9` | Quick-select element in category |
| `Left/Right` | Switch category |
| `Up/Down` | Select element |
| `/` | Search elements by name |
| `[` / `]` | Brush size (1-12) |
| `b` | Cycle brush shape |
| `+` / `-` | Zoom in/out |
| `Home/End/PgUp/PgDn` | Pan viewport |
| `Space` | Pause/Resume |
| `n` | Step one tick (when paused) |
| `f` | Cycle speed (1x/2x/4x) |
| `g` | Cycle gravity (Normal/Zero/Inverted) |
| `v` | Cycle view mode |
| `o` | Toggle grid overlay |
| `e` | Toggle eraser |
| `c` | Copy region (stamp) |
| `x` | Paste stamp |
| `s` / `l` | Save / Load |
| `S` | Cycle save slot (1-5) |
| `z` / `y` | Undo / Redo |
| `p` | Screenshot to file |
| `q` / `Esc` | Quit |

## Tech Stack

| Component | Technology |
|---|---|
| Language | Rust (2021 edition) |
| TUI Framework | [SuperLightTUI](https://crates.io/crates/superlighttui) `0.6` |
| Terminal Backend | crossterm `0.28` |
| RNG | rand `0.8` |
| Binary Size | ~2MB (release, LTO) |
| Dependencies | 3 crates total |

### Why SuperLightTUI?

SLT is an immediate-mode TUI library — no retained widget tree, no event loop boilerplate, no `App` trait. Your closure IS the app:

```rust
slt::run_with(config, |ui| {
    // describe UI every frame
    // SLT diffs and flushes only changed cells
});
```

This maps perfectly to a simulation loop: update physics, render grid, handle input — all in one closure, 30 times per second.

Key SLT features used in this project:
- **Half-block pixel rendering** via styled `▀` characters with independent fg/bg colors
- **Mouse support** for drawing, element selection, and brush preview
- **Flexbox layout** for canvas + sidebar arrangement
- **Sparkline widget** for particle count history
- **Double-buffer diffing** so only changed terminal cells are flushed

### Why Rust?

The simulation iterates a grid of 10,000+ cells at 30fps, updating pressure/heat/velocity fields, processing 50+ reaction rules, and rendering thousands of styled characters — all in a single thread.

Rust makes this practical:
- **No GC pauses** during simulation ticks
- **`Vec<Option<Particle>>`** with `Copy` particles for cache-friendly grid access
- **`cargo build --release` with LTO** produces a single static binary under 2MB
- **Zero `unsafe`** — the entire codebase, including SLT itself, is safe Rust

## Building

```bash
git clone https://github.com/subinium/powder-toy-slt
cd powder-toy-slt
cargo run --release
```

Requires Rust 1.74+ and a terminal with:
- True color support (most modern terminals)
- Mouse reporting
- Unicode support (for `▀` half-block characters)

Recommended terminal size: 120x40 or larger.

## Project Structure

```
src/
  elements.rs    (1,370 lines)  Element definitions, properties, categories
  simulation.rs  (2,394 lines)  Physics engine, reactions, field updates
  main.rs        (1,375 lines)  SLT app loop, rendering, input, UI
```

## Comparison with Original

| Feature | Original TPT | This Project |
|---|---|---|
| Language | C++ | Rust |
| Rendering | SDL2 GPU | Terminal (SLT) |
| Elements | 200+ | 151 |
| Physics Systems | 10+ | 9 |
| Resolution | 612x384 pixels | Terminal-dependent (~130x86 half-block) |
| Save/Load | Yes + Online | Yes (local, 5 slots) |
| Undo/Redo | No | Yes (64 frames) |
| Stamp | Yes | Yes |
| Zoom | Yes | Yes (1-4x) |
| Lua Scripting | Yes | No |
| Multiplayer | Limited | No |
| Binary Size | ~15MB | ~2MB |
| Dependencies | SDL2, Lua, zlib, ... | 3 crates |

## License

MIT
