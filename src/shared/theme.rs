#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeInfo {
    pub name: &'static str,
    pub display_name: &'static str,
    pub mode: ThemeMode,
    pub bg_hex: &'static str,
    pub fg_hex: &'static str,
    pub sidebar_bg: &'static str,
    pub accent_hex: &'static str,
    pub circadian_slot: Option<&'static str>,
}

pub const ALL_THEMES: &[ThemeInfo] = &[
    // Circadian Defaults First
    ThemeInfo {
        name: "solarized-light",
        display_name: "Solarized Light",
        mode: ThemeMode::Light,
        bg_hex: "#fdf6e3",
        fg_hex: "#657b83",
        sidebar_bg: "#eee8d5",
        accent_hex: "#268bd2",
        circadian_slot: Some("day"),
    },
    ThemeInfo {
        name: "gruvbox",
        display_name: "Gruvbox Dark",
        mode: ThemeMode::Dark,
        bg_hex: "#282828",
        fg_hex: "#ebdbb2",
        sidebar_bg: "#1d2021",
        accent_hex: "#d79921",
        circadian_slot: Some("dawn"),
    },
    ThemeInfo {
        name: "catppuccin",
        display_name: "Catppuccin Macchiato",
        mode: ThemeMode::Dark,
        bg_hex: "#1e1e2e",
        fg_hex: "#cdd6f4",
        sidebar_bg: "#181825",
        accent_hex: "#cba6f7",
        circadian_slot: Some("dusk"),
    },
    ThemeInfo {
        name: "osaka-jade",
        display_name: "Osaka Jade",
        mode: ThemeMode::Dark,
        bg_hex: "#1a221e",
        fg_hex: "#d4c8a0",
        sidebar_bg: "#1f2a24",
        accent_hex: "#509475",
        circadian_slot: Some("night"),
    },
    ThemeInfo {
        name: "rose-pine",
        display_name: "Rosé Pine",
        mode: ThemeMode::Dark,
        bg_hex: "#191724",
        fg_hex: "#e0def4",
        sidebar_bg: "#1f1d2e",
        accent_hex: "#ebbcba",
        circadian_slot: None,
    },
    // Light themes
    ThemeInfo {
        name: "catppuccin-latte",
        display_name: "Catppuccin Latte",
        mode: ThemeMode::Light,
        bg_hex: "#eff1f5",
        fg_hex: "#4c4f69",
        sidebar_bg: "#e6e9ef",
        accent_hex: "#8839ef",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "gruvbox-light",
        display_name: "Gruvbox Light",
        mode: ThemeMode::Light,
        bg_hex: "#fbf1c7",
        fg_hex: "#3c3836",
        sidebar_bg: "#ebdbb2",
        accent_hex: "#b57614",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "tokyo-night-day",
        display_name: "Tokyo Night Day",
        mode: ThemeMode::Light,
        bg_hex: "#e1e2e7",
        fg_hex: "#3760bf",
        sidebar_bg: "#d5d6db",
        accent_hex: "#2e7de9",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "rose-pine-dawn",
        display_name: "Rosé Pine Dawn",
        mode: ThemeMode::Light,
        bg_hex: "#faf4ed",
        fg_hex: "#575279",
        sidebar_bg: "#f2e9de",
        accent_hex: "#d7827e",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "one-light",
        display_name: "One Light",
        mode: ThemeMode::Light,
        bg_hex: "#fafafa",
        fg_hex: "#383a42",
        sidebar_bg: "#f0f0f0",
        accent_hex: "#4078f2",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "kanagawa-lotus",
        display_name: "Kanagawa Lotus",
        mode: ThemeMode::Light,
        bg_hex: "#f2ecbc",
        fg_hex: "#545464",
        sidebar_bg: "#e7e1b5",
        accent_hex: "#4d699b",
        circadian_slot: None,
    },
    // Dark themes
    ThemeInfo {
        name: "solarized",
        display_name: "Solarized Dark",
        mode: ThemeMode::Dark,
        bg_hex: "#002b36",
        fg_hex: "#839496",
        sidebar_bg: "#073642",
        accent_hex: "#268bd2",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "tokyo-night",
        display_name: "Tokyo Night",
        mode: ThemeMode::Dark,
        bg_hex: "#1a1b26",
        fg_hex: "#c0caf5",
        sidebar_bg: "#16161e",
        accent_hex: "#7aa2f7",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "dracula",
        display_name: "Dracula",
        mode: ThemeMode::Dark,
        bg_hex: "#282a36",
        fg_hex: "#f8f8f2",
        sidebar_bg: "#21222c",
        accent_hex: "#bd93f9",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "nord",
        display_name: "Nord",
        mode: ThemeMode::Dark,
        bg_hex: "#2e3440",
        fg_hex: "#d8dee9",
        sidebar_bg: "#242933",
        accent_hex: "#88c0d0",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "one-dark",
        display_name: "One Dark",
        mode: ThemeMode::Dark,
        bg_hex: "#282c34",
        fg_hex: "#abb2bf",
        sidebar_bg: "#21252b",
        accent_hex: "#61afef",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "kanagawa",
        display_name: "Kanagawa",
        mode: ThemeMode::Dark,
        bg_hex: "#1f1f28",
        fg_hex: "#dcd7ba",
        sidebar_bg: "#16161d",
        accent_hex: "#7e9cd8",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "vesper",
        display_name: "Vesper",
        mode: ThemeMode::Dark,
        bg_hex: "#101010",
        fg_hex: "#ffffff",
        sidebar_bg: "#161616",
        accent_hex: "#ffc799",
        circadian_slot: None,
    },
    ThemeInfo {
        name: "terminal",
        display_name: "Terminal Default",
        mode: ThemeMode::Dark,
        bg_hex: "#000000",
        fg_hex: "#ffffff",
        sidebar_bg: "#121212",
        accent_hex: "#00ff00",
        circadian_slot: None,
    },
];

pub fn find_theme(name: &str) -> Option<&'static ThemeInfo> {
    let lower = name.to_ascii_lowercase();
    ALL_THEMES.iter().find(|t| t.name.to_ascii_lowercase() == lower)
}

pub fn circadian_theme_for_slot(slot: &str) -> &'static ThemeInfo {
    match slot.to_ascii_lowercase().as_str() {
        "day" => find_theme("solarized-light").unwrap(),
        "dawn" => find_theme("gruvbox").unwrap(),
        "dusk" => find_theme("catppuccin").unwrap(),
        "night" => find_theme("rose-pine").unwrap(),
        _ => find_theme("catppuccin").unwrap(),
    }
}

pub fn counterpart_theme(theme: &ThemeInfo) -> Option<&'static ThemeInfo> {
    let name = theme.name;
    match name {
        "catppuccin" => find_theme("catppuccin-latte"),
        "catppuccin-latte" => find_theme("catppuccin"),
        "solarized" => find_theme("solarized-light"),
        "solarized-light" => find_theme("solarized"),
        "gruvbox" => find_theme("gruvbox-light"),
        "gruvbox-light" => find_theme("gruvbox"),
        "tokyo-night" => find_theme("tokyo-night-day"),
        "tokyo-night-day" => find_theme("tokyo-night"),
        "rose-pine" => find_theme("rose-pine-dawn"),
        "rose-pine-dawn" => find_theme("rose-pine"),
        "one-dark" => find_theme("one-light"),
        "one-light" => find_theme("one-dark"),
        "kanagawa" => find_theme("kanagawa-lotus"),
        "kanagawa-lotus" => find_theme("kanagawa"),
        // Themes with no explicit light variant default to solarized-light if dark, or catppuccin if light
        _ => {
            if theme.mode == ThemeMode::Dark {
                find_theme("solarized-light")
            } else {
                find_theme("catppuccin")
            }
        }
    }
}
