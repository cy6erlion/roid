use crate::RoidAdapder;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;

/// Functionality for installing Android Apps on different types of devices
pub struct Install;

impl RoidAdapder for Install {
    /// The build cli subcommand
    fn cmd<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("install")
            .about("Install an APK on a device or emulator")
            .arg(
                Arg::with_name("apk")
                    .long("apk")
                    .short("a")
                    .takes_value(true)
                    .help("Install project in current directory on a device"),
            )
    }
    /// check build commands from command line and decide what to do
    fn process_cmd(matches: ArgMatches<'static>) {
        match matches.subcommand_matches("install") {
            Some(flag) => match flag.value_of("apk") {
                Some(p) => Install::apk(p).unwrap(),
                None => (),
            },
            None => (),
        }
    }
}

impl Install {
    /// Install project in current directory on a device
    pub fn apk(apk_path: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            println!("{}", apk_path);
            Command::new("adb")
                .arg("-d")
                .arg("install")
                .arg(apk_path)
                .status()
                .expect("failed to emulate Project");

            Ok(())
        }
    }
}
