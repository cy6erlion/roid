//! # roid
//! Android Developer Toolkit

extern crate clap;
mod build;
mod config;
mod device;
mod install;
mod project;
use clap::{App, ArgMatches};

const LOGO: &str = r#"
               (
 (        (    )\ )
 )(    (  )\  (()/(
(()\   )\((_)  ((_))
 ((_) ((_)(_)  _| |
| '_|/ _ \| |/ _` |
|_|  \___/|_|\__,_|
"#;

/// Adapder for Roid commands and features.
pub trait RoidAdapder {
    /// Get commands from the CLI (Command Line Interface)
    fn cmd<'a, 'b>() -> App<'a, 'b>;

    /// Process the commands
    fn process_cmd(matches: ArgMatches<'static>);
}

fn main() {
    // register commands
    let matches = App::new("roid")
        .version("0.1.0")
        .about("Android Developer Toolkit")
        .before_help(LOGO)
        .subcommand(project::Project::cmd())
        .subcommand(build::Build::cmd())
        .subcommand(install::Install::cmd())
        .subcommand(device::Device::cmd())
        .get_matches();

    // process commands
    project::Project::process_cmd(matches.clone());
    build::Build::process_cmd(matches.clone());
    install::Install::process_cmd(matches.clone());
    device::Device::process_cmd(matches);
}
