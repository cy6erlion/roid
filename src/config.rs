use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Config {
    /// Directory to find Android project templates
    pub templates: String,
}
