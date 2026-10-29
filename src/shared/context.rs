use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct HerdrContext {
    pub workspace_id: Option<String>,
    pub tab_id: Option<String>,
    pub pane_id: Option<String>,
    pub invocation_source: Option<String>,
}

impl HerdrContext {
    pub fn from_env() -> Self {
        match env::var("HERDR_PLUGIN_CONTEXT_JSON") {
            Ok(json_str) => serde_json::from_str(&json_str).unwrap_or_default(),
            Err(_) => HerdrContext::default(),
        }
    }
}
