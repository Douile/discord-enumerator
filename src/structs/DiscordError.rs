use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordError {
    pub code: u32,
    pub message: String
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiscordError {}: {}", self.code, self.message)
    }
}

impl std::error::Error for DiscordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
