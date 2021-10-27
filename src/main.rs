//! # roid
//! Android Developer Toolkit

extern crate clap;
mod build;
pub mod config;
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
    fn process_cmd(matches: ArgMatches<'static>, conf: &config::Config);
}

fn main() -> std::io::Result<()> {
    let conf = config::Config::read_config();

    // register commands
    let matches = App::new("roid")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Android Developer Toolkit")
        .before_help(LOGO)
        .subcommand(project::Project::cmd())
        .subcommand(build::Build::cmd())
        .subcommand(install::Install::cmd())
        .subcommand(device::Device::cmd())
        .get_matches();

    // process commands
    project::Project::process_cmd(matches.clone(), &conf);
    build::Build::process_cmd(matches.clone(), &conf);
    install::Install::process_cmd(matches.clone(), &conf);
    device::Device::process_cmd(matches, &conf);

    Ok(())
}
