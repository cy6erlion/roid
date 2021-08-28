use crate::config::Config;
use crate::RoidAdapder;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;

/// Functionality for building Android Projects
pub struct Build;

impl RoidAdapder for Build {
    /// The build cli subcommand
    fn cmd<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("build")
            .about("Build an Android project")
            .arg(
                Arg::with_name("debug")
                    .long("debug")
                    .short("d")
                    .help("Build project in debug mode"),
            )
            .arg(
                Arg::with_name("release")
                    .long("release")
                    .short("r")
                    .help("Build project in release mode"),
            )
    }
    /// check build commands from command line and decide what to do
    fn process_cmd(matches: ArgMatches<'static>, conf: &Config) {
        match matches.subcommand_matches("build") {
            Some(flag) => {
                let cmd = Config::get_gradle(conf);

                // check if debug build
                if flag.is_present("debug") {
                    Build::debug(&cmd).unwrap();
                }

                // check if release build
                if flag.is_present("release") {
                    Build::release(&cmd).unwrap();
                }
            }
            None => (),
        }
    }
}

impl Build {
    /// Build an Android project in debug mode
    pub fn debug(cmd: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            Command::new(cmd)
                .arg("assembleDebug")
                .status()
                .expect("failed to build Project in debug mode");

            Ok(())
        }
    }
    /// Build an Android project in release mode
    pub fn release(cmd: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            Command::new(cmd)
                .arg("assemble")
                .status()
                .expect("failed to build Project in release mode");

            Ok(())
        }
    }
}
