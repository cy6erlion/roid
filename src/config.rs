use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

/// Default template repo url, can be set in Roid.toml file
const TEMPLATES_URL: &str = "https://github.com/cy6erlion/android-project-templates.git";

#[derive(Deserialize)]
pub struct Config {
    /// URL of Android project templates repo
    pub templates: Option<String>,

    /// Path to gradle command
    pub gradle: Option<String>,

    /// Path to emulator cli tool
    pub emulator: Option<String>,

    /// Path to adb command
    pub adb: Option<String>,
}

/// Configuration utility functions
impl Config {
    pub fn read_config() -> Config {
        // Read Config file
        let mut file_path;

        if let Some(p) = dirs::config_dir() {
            file_path = p;
        } else {
            panic!("Config directory not found");
        }
        file_path.push("Roid.toml");

        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();
        config
    }

    /// Get templates repo url
    pub fn get_templates(config: &Config) -> &str {
        // Check config for templates repo url
        if let Some(url) = &config.templates {
            url
        } else {
            TEMPLATES_URL
        }
    }

    /// Get gradle command
    pub fn get_gradle(config: &Config) -> &str {
        // default command
        let mut cmd = "gradle";

        // Check config for gradle command
        if let Some(c) = &config.gradle {
            cmd = c;
        }

        cmd
    }

    /// Get emulator command
    pub fn get_emulator(config: &Config) -> &str {
        // default command
        let mut cmd = "emulator";

        // Check config for emulator command
        if let Some(c) = &config.emulator {
            cmd = c;
        }

        cmd
    }

    /// Get adb command
    pub fn get_adb(config: &Config) -> &str {
        // default command
        let mut cmd = "adb";

        // Check config for adb command
        if let Some(c) = &config.adb {
            cmd = c;
        }

        cmd
    }
}
