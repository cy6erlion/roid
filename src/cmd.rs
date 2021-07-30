use clap::{App, Arg, ArgMatches, SubCommand};

pub fn matches() -> ArgMatches<'static> {
    App::new("roid")
        .version("0.1.0")
        .about("Android Developer Toolkit")
        // The `new` subcommand
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new Android project")
                // an Android project with no activity
                .arg(
                    Arg::with_name("no-activity")
                        .long("none")
                        .short("n")
                        .takes_value(true)
                        .help("Create a new Android project with no activity"),
                )
                // an Android project with an empty activity
                .arg(
                    Arg::with_name("empty-activity")
                        .long("empty")
                        .short("e")
                        .takes_value(true)
                        .help("Create a new Android project with an Empty Activity"),
                )
                // an Android project with a basic activity
                .arg(
                    Arg::with_name("basic-activity")
                        .long("basic")
                        .short("b")
                        .takes_value(true)
                        .help("Create a new Android project with a Basic Activity"),
                )
                // an Android project with a Bottom Navigation activity
                .arg(
                    Arg::with_name("bottom-nav-activity")
                        .long("bottom-nav")
                        .short("v")
                        .takes_value(true)
                        .help("Create a new Android project with a Bottom Navigation Activity"),
                ),
        )
        .get_matches()
}
