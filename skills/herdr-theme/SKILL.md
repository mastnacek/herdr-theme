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
# 1. Interactive Ratatui manager TUI (Tab 1: Themes, Tab 2: Settings, Tab 3: Solar Clock)
herdr_theme-switcher picker

# 2. Interactive settings configuration form directly
herdr_theme-switcher settings --tui
# or: herdr_theme-switcher settings-tui

# 3. Directly apply a specific theme
herdr_theme-switcher apply <theme-name>
# Examples:
# herdr_theme-switcher apply solarized-light
# herdr_theme-switcher apply catppuccin

# 4. Toggle between dark and light counterparts
herdr_theme-switcher toggle

# 5. Sync theme to current solar elevation (NOAA astronomical phase)
herdr_theme-switcher circadian [--force]

# 6. Continuous background watcher (monitors and auto-switches theme when phase changes)
herdr_theme-switcher watch [--interval 60]

# 7. Start/stop background daemon process (detached, no window)
herdr_theme-switcher start-daemon
herdr_theme-switcher stop-daemon
herdr_theme-switcher status-daemon

# 8. View or inspect plugin settings and configuration file path
herdr_theme-switcher settings [--path]

# 9. List all 18 supported themes with mode and hex colors
herdr_theme-switcher list

# 10. Display currently configured theme
herdr_theme-switcher current

# 11. Reset terminal skin to profile defaults
herdr_theme-switcher reset-skin
```

## Interactive Full-Featured TUI (`picker` / `settings-tui`)

The interactive Ratatui TUI features 3 synchronized tabs:
- **Tab 1: Themes (`1`)**:
  - Live list of all 18 themes with color swatch preview cards, hex color breakdown, and simulated terminal canvas.
  - Controls: `↑`/`↓`/`j`/`k` (select), `Enter` (apply immediately), `f` (filter all/dark/light), `c` (sync circadian).
- **Tab 2: Settings (`2`)**:
  - Full configuration form: customize Dawn, Day, Dusk, and Night themes from all 18 options with live swatches.
  - Select City presets (Prague, London, Berlin, Paris, New York, San Francisco, Tokyo, Sydney) or adjust custom latitude/longitude coordinates.
  - Set watcher cycle interval (10s, 30s, 60s, 120s, 300s, 600s).
  - Toggle host terminal skinning (OSC 10/11) and desktop notifications.
  - Start/stop background daemon directly from the UI.
  - Controls: `↑`/`↓` (navigate fields), `←`/`→`/`h`/`l`/`Space` (change values/toggle), `s` / `Enter` (save to `settings.toml`), `r` (resync theme now).
- **Tab 3: Solar Clock (`3`)**:
  - Real-time astronomical ephemeris: UTC/Local time, observer position, exact NOAA sun elevation angle in degrees, current phase, and ASCII horizon gauge.

Open inside Herdr as a floating popup:
```bash
# Open theme picker popup:
herdr plugin pane open --plugin herdr.theme-switcher --entrypoint picker-win

# Open settings configuration popup:
herdr plugin pane open --plugin herdr.theme-switcher --entrypoint settings-win
```

## Configuration Settings

The plugin automatically creates and manages its own persistent configuration file in Herdr's plugin config directory:
- **Windows**: `%APPDATA%\herdr\plugins\config\herdr.theme-switcher\settings.toml`
- **Linux/macOS**: `~/.config/herdr/plugins/config/herdr.theme-switcher/settings.toml`

### Settings schema (`settings.toml`)

```toml
[location]
# Latitude and Longitude for NOAA solar elevation calculations
# Default: Prague, Czech Republic
latitude = 50.0755
longitude = 14.4378

[circadian]
# Customizable theme assigned to each solar phase
# Choose any of the 18 themes from 'herdr_theme-switcher list'
dawn = "gruvbox"           # -12° to -0.83° (rising sun)
day = "solarized-light"    # > -0.83° (sun above horizon)
dusk = "catppuccin"        # -0.83° to -12° (setting sun)
night = "rose-pine"        # < -12° (astronomical night)

[daemon]
# Monitoring frequency in seconds
interval_seconds = 60

[terminal]
# Automatically send OSC 10/11 escape sequences to skin host terminal
skin_terminal = true

# Show desktop notification on automatic theme change
notify = true
```

## Automated Monitoring Mechanisms

1. **Continuous Daemon (`watch`)**:
   Runs as a background daemon loop every 60s (or custom interval via `--interval <sec>`). When solar elevation crosses a threshold (Dawn, Day, Dusk, Night), it automatically updates `config.toml`, skins the host terminal via OSC 10/11, reloads Herdr server, and emits a desktop notification.

2. **Event-driven via Herdr (`tab.created`, `pane.created`, `startup`)**:
   Registered in `herdr-plugin.toml`. Whenever Herdr boots, or whenever a user opens a new tab or pane, Herdr invokes `circadian`. If the theme is already in sync with the current solar slot, it completes in <1ms without disk writes or reloads; if the phase has shifted, it immediately switches the theme.


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
