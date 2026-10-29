---
name: herdr-theme
description: Manage and synchronize Herdr multiplexer themes, terminal skinning (OSC 10/11), and circadian solar lighting rotation.
---

# Herdr Theme Switcher Skill (`herdr.theme-switcher`)

Use this skill when interacting with Herdr themes, setting the terminal skin, or automating circadian lighting transitions for coding agents in Herdr.

## Architecture & Layout

This plugin follows Vertical Slice Architecture (VSA) in Rust and Ratatui:
- `src/main.rs`: Composition root and CLI argument dispatcher.
- `src/shared/`: Shared kernel (Herdr CLI client, atomic TOML config updater, NOAA solar calculations, OSC terminal emitter).
- `src/slices/picker/`: Interactive Ratatui popup terminal UI.
- `src/slices/apply/`: Direct CLI application, listing, and dark/light toggling.
- `src/slices/circadian/`: Solar position rotation (dawn, day, dusk, night).

## Available CLI Commands

When invoking the compiled binary directly (`herdr_theme-switcher` / `herdr_theme-switcher.exe`):

```bash
# 1. Interactive Ratatui popup theme picker
herdr_theme-switcher picker

# 2. Directly apply a specific theme
herdr_theme-switcher apply <theme-name>
# Examples:
# herdr_theme-switcher apply solarized-light
# herdr_theme-switcher apply catppuccin

# 3. Toggle between dark and light counterparts
herdr_theme-switcher toggle

# 4. Sync theme to current solar elevation (NOAA astronomical phase)
herdr_theme-switcher circadian

# 5. List all 18 supported themes with mode and hex colors
herdr_theme-switcher list

# 6. Display currently configured theme
herdr_theme-switcher current

# 7. Reset terminal skin to profile defaults
herdr_theme-switcher reset-skin
```

## Herdr Multiplexer Integration

Declared in `herdr-plugin.toml`:

- **Popup Pane**:
  - `id`: `picker` (Unix) / `picker-win` (Windows)
  - `placement`: `popup` (width 75%, height 20)
  - Opened via: `herdr plugin pane open --plugin herdr.theme-switcher --entrypoint picker-win`

- **Actions**:
  - `herdr.theme-switcher.toggle-win`: Toggles between dark and light themes.
  - `herdr.theme-switcher.circadian-win`: Triggers immediate circadian solar rotation.
  - `herdr.theme-switcher.list-win`: Lists supported themes in Herdr logs.

- **Startup Hook**:
  - Runs `circadian` on Herdr startup/restore so the multiplexer automatically begins with the appropriate lighting phase for the time of day.

## Terminal Skinning Details

- **OSC 11**: `\x1b]11;#RRGGBB\x07` sets the host terminal background.
- **OSC 10**: `\x1b]10;#RRGGBB\x07` sets the host terminal text/foreground.
- **OSC 111 / 110**: Resets background and foreground to host profile defaults.
- **Herdr Live Reload**: Automatically updates `config.toml` (`[theme] name = ...`, `[theme.custom] sidebar_bg = ...`) and invokes `herdr server reload-config`.
