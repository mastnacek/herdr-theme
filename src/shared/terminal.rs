use crate::shared::theme::ThemeInfo;
use std::io::{self, Write};

/// Emits OSC 10 and OSC 11 sequences to set terminal text and background skin.
pub fn emit_theme_skin(theme: &ThemeInfo) -> io::Result<()> {
    let mut stdout = io::stdout();
    // OSC 11: Set background color
    write!(stdout, "\x1b]11;{}\x07", theme.bg_hex)?;
    // OSC 10: Set foreground / font color
    write!(stdout, "\x1b]10;{}\x07", theme.fg_hex)?;
    stdout.flush()
}

/// Resets terminal background (OSC 111) and foreground (OSC 110) to profile defaults.
pub fn reset_terminal_skin() -> io::Result<()> {
    let mut stdout = io::stdout();
    write!(stdout, "\x1b]111\x07\x1b]110\x07")?;
    stdout.flush()
}
