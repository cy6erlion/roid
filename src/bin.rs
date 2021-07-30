//! # roid
//! Android Developer Toolkit

mod cmd;
mod config;
mod project;

use cmd::matches;
use config::Config;
use project::{Project, TemplateBranch};
use std::fs;

extern crate clap;

fn main() {
    let matches = matches();

    //////////////// Config subcommand
    match matches.subcommand_matches("config") {
        Some(config) => {
            let config_file = config.value_of("set").unwrap_or("default.conf");
            let data = fs::read_to_string(config_file).expect("Unable to read file");
            let config: Config = toml::from_str(&data).unwrap();
            println!("{}", config.templates);
        }
        None => (),
    }

    //////////////// New subcommand
    match matches.subcommand_matches("new") {
        Some(new_project) => {
            // Create a new Android Project with No activity
            if new_project.is_present("no-activity") {
                match new_project.value_of("no-activity") {
                    Some(name) => Project::create(name, TemplateBranch::NoActivity).unwrap(),
                    None => (),
                }
            }

            // Create a new Android Project with an empty activity
            if new_project.is_present("empty-activity") {
                match new_project.value_of("empty-activity") {
                    Some(name) => Project::create(name, TemplateBranch::EmptyActivity).unwrap(),
                    None => (),
                }
            }

            // Create a new Android Project with a basic activity
            if new_project.is_present("basic-activity") {
                match new_project.value_of("basic-activity") {
                    Some(name) => Project::create(name, TemplateBranch::BasicActivity).unwrap(),
                    None => (),
                }
            }

            // Create a new Android Project with a bottom navigation activity
            if new_project.is_present("bottom-nav-activity") {
                match new_project.value_of("bottom-nav-activity") {
                    Some(name) => Project::create(name, TemplateBranch::BottomNavActivity).unwrap(),
                    None => (),
                }
            }
        }
        None => (),
    }
}
