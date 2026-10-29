use std::env;
use std::process::Command;

pub struct HerdrClient {
    bin: String,
}

impl HerdrClient {
    pub fn new() -> Self {
        Self {
            bin: env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string()),
        }
    }

    pub fn reload_server(&self) -> bool {
        let status = Command::new(&self.bin)
            .args(["server", "reload-config"])
            .status();

        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }

    pub fn notify(&self, title: &str, body: &str) {
        let _ = Command::new(&self.bin)
            .args(["notification", "show", title, "--body", body])
            .status();
    }
}
