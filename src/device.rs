use crate::config::Config;
use crate::RoidAdapder;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;

/// Functionality for dealing with Android devices
pub struct Device;

impl RoidAdapder for Device {
    /// The build cli subcommand
    fn cmd<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("device")
            .about("Android device management")
            .arg(
                Arg::with_name("list")
                    .long("list")
                    .short("l")
                    .help("List all available devices and emulators"),
            )
            .arg(
                Arg::with_name("emulator")
                    .long("emulator")
                    .short("e")
                    .takes_value(true)
                    .help("Launch an emulator"),
            )
    }
    /// check build commands from command line and decide what to do
    fn process_cmd(matches: ArgMatches<'static>, conf: &Config) {
        match matches.subcommand_matches("device") {
            Some(flag) => {
                let emulator = Config::get_emulator(&conf);
                let adb = Config::get_adb(&conf);

                if flag.is_present("list") {
                    Device::list(emulator, adb).unwrap()
                }

                if flag.is_present("emulator") {
                    match flag.value_of("emulator") {
                        Some(name) => Device::emulator(name, emulator).unwrap(),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}

impl Device {
    /// List all available Android devices and emulators
    pub fn list(emulator: &str, adb: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            println!("\n-------- ANDROID DEVICES -------");
            println!("\nList of available emulators");
            Command::new(emulator)
                .arg("-list-avds")
                .status()
                .expect("failed to list emulators");
            println!("");
            Command::new(adb)
                .arg("devices")
                .status()
                .expect("failed to list attached devices");
            println!("--------------------------------");
            Ok(())
        }
    }

    /// Launch an emulator
    pub fn emulator(name: &str, emulator: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            Command::new(emulator)
                .arg("-avd")
                .arg(name)
                .status()
                .expect("failed to list emulators");
            Ok(())
        }
    }
}
