# Herdr Theme Switcher (`herdr.theme-switcher`)

High-performance, native Rust & Ratatui theme switcher and terminal skin manager for the [Herdr multiplexer](https://github.com/alexarthurs/herdr).

Provides full-system circadian lighting rotation, interactive theme browser popup with live palette preview, dark/light toggle, and host terminal background/text skinning via OSC 10 / OSC 11 escape sequences.

---

## Features

1. **Terminal Skin Synchronization (OSC 10 / OSC 11)**:
   - Emits standard ANSI OSC 11 (`\x1b]11;#RRGGBB\x07`) to skin host terminal background.
   - Emits standard ANSI OSC 10 (`\x1b]10;#RRGGBB\x07`) to skin host terminal text/foreground.
   - Restores default terminal profile colors via `reset-skin` (OSC 110 / OSC 111).
   - Compatible with Windows Terminal, WezTerm, Alacritty, Ghostty, Kitty, iTerm2.

2. **Live Herdr Multiplexer Reload**:
   - Updates `config.toml` (`[theme] name = "..."`, `[theme.custom] sidebar_bg = "..."`).
   - Disables `auto_switch` so Herdr honors the selected palette.
   - Preserves all existing comments, keybindings, and custom section configurations using `toml_edit`.
   - Fires `herdr server reload-config` for instant hot-reload across all windows and panes.

3. **Interactive Ratatui Theme Picker (`picker`)**:
   - Out-of-process popup pane (`placement = "popup"`, 75% width, 20 rows).
   - Two-column view: categorized list with `[Dark]` / `[Light]` badges on the left, live palette breakdown and simulated terminal pane on the right.
   - Fast keyboard navigation: `↑` / `↓` / `j` / `k` to browse, `Tab` to cycle Dark/Light filters, `Enter` to apply live, `c` to sync circadian, `q` / `Esc` to close.

4. **Circadian Solar Rotation (`circadian`)**:
   - Calculates solar elevation using NOAA astronomical equations based on latitude and longitude (default Prague: 50.0755°N, 14.4378°E; customizable via `LATITUDE` / `LONGITUDE`).
   - Automatically selects the optimal lighting phase:
     - **Dawn** (twilight rising): `gruvbox` (warm, soft contrast)
     - **Day** (sun above horizon): `solarized-light` (clear daylight legibility)
     - **Dusk** (twilight setting): `catppuccin` (mellow dark)
     - **Night** (deep dark): `rose-pine` (low strain, restful deep night)
   - Executed automatically on Herdr server `[[startup]]` and restoration.

5. **CLI Actions**:
   - `apply <theme>`: Apply any built-in theme directly.
   - `toggle`: Toggle between light and dark counterparts (e.g. `catppuccin` ↔ `catppuccin-latte`, `solarized` ↔ `solarized-light`).
   - `list`: Show all supported themes with mode, background, foreground, and sidebar colors.
   - `current`: Show currently active Herdr theme from config.

---

## Architecture (Vertical Slice Architecture — VSA)

This plugin strictly adheres to Vertical Slice Architecture:

```text
herdr-theme/
├── herdr-plugin.toml           # Manifest declaring actions, startup hooks, and popup pane
├── Cargo.toml                  # Fast, stripped release profile and dependencies
└── src/
    ├── main.rs                 # Composition root & CLI dispatch
    ├── shared/                 # Kernel / Shared (Zero slice knowledge)
    │   ├── mod.rs
    │   ├── client.rs           # Herdr CLI client (server reload-config, notifications)
    │   ├── config.rs           # Atomic, format-preserving TOML reader and updater
    │   ├── context.rs          # HERDR_PLUGIN_CONTEXT_JSON deserializer
    │   ├── solar.rs            # NOAA solar elevation and slot calculation
    │   ├── terminal.rs         # OSC 10 & 11 escape sequence emitter
    │   └── theme.rs            # Theme definitions, palettes, and counterparts
    └── slices/                 # Isolated Feature Slices (Never import each other)
        ├── mod.rs
        ├── picker/             # Interactive Ratatui popup theme picker
        │   ├── mod.rs          # Popup event loop & crossterm terminal runner
        │   ├── state.rs        # Picker state, filtering, and selection
        │   └── ui.rs           # Ratatui rendering & palette preview widgets
        ├── apply/              # Direct action & CLI slice
        │   └── mod.rs          # apply, toggle, list, current commands
        └── circadian/          # Solar elevation rotation slice
            └── mod.rs          # Solar slot sync and notification
```

---

## Supported Herdr Themes

| Theme Name | Mode | Background | Foreground | Sidebar BG | Circadian Slot |
|---|---|---|---|---|---|
| `solarized-light` | Light | `#fdf6e3` | `#657b83` | `#eee8d5` | Day |
| `gruvbox` | Dark | `#282828` | `#ebdbb2` | `#1d2021` | Dawn |
| `catppuccin` | Dark | `#1e1e2e` | `#cdd6f4` | `#181825` | Dusk |
| `rose-pine` | Dark | `#191724` | `#e0def4` | `#1f1d2e` | Night |
| `catppuccin-latte` | Light | `#eff1f5` | `#4c4f69` | `#e6e9ef` | - |
| `gruvbox-light` | Light | `#fbf1c7` | `#3c3836` | `#ebdbb2` | - |
| `tokyo-night-day` | Light | `#e1e2e7` | `#3760bf` | `#d5d6db` | - |
| `rose-pine-dawn` | Light | `#faf4ed` | `#575279` | `#f2e9de` | - |
| `one-light` | Light | `#fafafa` | `#383a42` | `#f0f0f0` | - |
| `kanagawa-lotus` | Light | `#f2ecbc` | `#545464` | `#e7e1b5` | - |
| `solarized` | Dark | `#002b36` | `#839496` | `#073642` | - |
| `tokyo-night` | Dark | `#1a1b26` | `#c0caf5` | `#16161e` | - |
| `dracula` | Dark | `#282a36` | `#f8f8f2` | `#21222c` | - |
| `nord` | Dark | `#2e3440` | `#d8dee9` | `#242933` | - |
| `one-dark` | Dark | `#282c34` | `#abb2bf` | `#21252b` | - |
| `kanagawa` | Dark | `#1f1f28` | `#dcd7ba` | `#16161d` | - |
| `vesper` | Dark | `#101010` | `#ffffff` | `#161616` | - |
| `terminal` | Dark | `#000000` | `#ffffff` | `#121212` | - |

---

## Installation & Linking in Herdr

```bash
# In the plugin directory:
cargo build --release

# Link plugin to Herdr:
herdr plugin link .

# List registered actions:
herdr plugin action list --plugin herdr.theme-switcher

# Open Theme Picker popup pane:
herdr plugin pane open --plugin herdr.theme-switcher --entrypoint picker-win

# Toggle Light/Dark mode via Herdr action:
herdr plugin action invoke herdr.theme-switcher.toggle-win
```
