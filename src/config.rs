use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

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
        let mut file = File::open("/home/seestem/.config/Roid.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();
        config
    }

    /// Get templates repo url
    pub fn get_templates(config: &Config) -> &str {
        // default repo
        let mut url = "https://gitlab.com/seestem/android-project-templates.git";

        // Check config for templates repo url
        if let Some(u) = &config.templates {
            url = u;
        }

        url
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
