# itop

A terminal system monitor for macOS Apple Silicon, written in Rust.

Displays CPU, memory, swap, per-core usage, and **real-time MPS/Metal GPU utilization** — all in a beautiful Catppuccin-themed TUI.

```
⚡ itop  system monitor  [q] quit  [r] refresh
┌─ CPU  14 cores ─────────────────────────────┐
│ ████████████░░░░░░░░░░░░░░░░░░  24.3%       │
└─────────────────────────────────────────────┘
┌─ MEM  memory ───────────────────────────────┐
│ ████████████████████░░░░░░░░░░  9.2 / 24 GB │
└─────────────────────────────────────────────┘
```

## Features

- **CPU** — global gauge + per-core bars + 60-second history chart
- **Memory / Swap** — gauges + history chart (GB display)
- **GPU (MPS)** — live utilization gauge + history chart via `ioreg` (no `sudo` needed)
- **System info** — hostname, OS version, kernel, uptime
- Catppuccin Mocha color palette, Braille-resolution sparklines
- Keyboard: `q` / `Esc` to quit, `r` to force refresh

## Requirements

- macOS (Apple Silicon recommended, Intel works without GPU stats)
- Rust 1.85+ (edition 2024)

## Install

### From source

```bash
git clone https://github.com/spike/itop.git
cd itop
cargo install --path .
```

After install the binary lands in `~/.cargo/bin/itop`. Make sure `~/.cargo/bin` is in your `$PATH`.

### Manual binary install

```bash
cargo build --release
sudo cp target/release/itop /usr/local/bin/
```

## Usage

```bash
itop
```

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `r` | Force refresh |

## GPU Notes

GPU utilization is read from `IOAccelerator` via `ioreg`, which exposes the `Device Utilization %` field from Apple's Metal Performance Shaders runtime.  No root privileges required.  If you see the "no IOAccelerator found" message, try running `ioreg -r -d 1 -c IOAccelerator` to verify your system exposes this data.

## Project Structure

```
src/
├── main.rs              # event loop + terminal setup
├── app.rs               # App state + update logic
├── gpu.rs               # ioreg-based MPS GPU metrics
├── theme.rs             # Catppuccin color constants
└── ui/
    ├── mod.rs           # layout orchestration
    ├── header.rs
    ├── cpu.rs           # gauge, chart, per-core
    ├── memory.rs        # mem/swap gauge + chart
    ├── gpu_panel.rs     # GPU gauge + history
    └── sysinfo_panel.rs
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui-org/ratatui) | TUI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal backend |
| [sysinfo](https://github.com/GuillaumeGomez/sysinfo) | CPU / memory metrics |

## License

MIT
