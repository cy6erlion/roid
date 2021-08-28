use crate::config::Config;
use crate::RoidAdapder;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;
use std::process::Command;

pub struct Project;

impl RoidAdapder for Project {
    /// The new cli subcommand
    fn cmd<'a, 'b>() -> App<'a, 'b> {
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
            )
    }

    /// Check the cli commands provided and decide what to do
    fn process_cmd(matches: ArgMatches<'static>, conf: &Config) {
        let url = Config::get_templates(conf);

        match matches.subcommand_matches("new") {
            Some(new_project) => {
                // Create a new Android Project with No activity
                if new_project.is_present("no-activity") {
                    match new_project.value_of("no-activity") {
                        Some(name) => {
                            Project::create(name, TemplateBranch::NoActivity, url).unwrap()
                        }
                        None => (),
                    }
                }

                // Create a new Android Project with an empty activity
                if new_project.is_present("empty-activity") {
                    match new_project.value_of("empty-activity") {
                        Some(name) => {
                            Project::create(name, TemplateBranch::EmptyActivity, url).unwrap()
                        }
                        None => (),
                    }
                }

                // Create a new Android Project with a basic activity
                if new_project.is_present("basic-activity") {
                    match new_project.value_of("basic-activity") {
                        Some(name) => {
                            Project::create(name, TemplateBranch::BasicActivity, url).unwrap()
                        }
                        None => (),
                    }
                }

                // Create a new Android Project with a bottom navigation activity
                if new_project.is_present("bottom-nav-activity") {
                    match new_project.value_of("bottom-nav-activity") {
                        Some(name) => {
                            Project::create(name, TemplateBranch::BottomNavActivity, url).unwrap()
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}

impl Project {
    /// Create a project
    pub fn create(project_name: &str, template: TemplateBranch, url: &str) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            let branch = get_template_branch(template);

            Command::new("git")
                .arg("clone")
                .arg("--single-branch")
                .arg("--branch")
                .arg(branch)
                .arg(url)
                .arg(project_name)
                .status()
                .expect("failed to execute git clone");

            reset_project(project_name).unwrap();

            Ok(())
        }
    }
}

pub enum TemplateBranch {
    BasicActivity,
    BottomNavActivity,
    EmptyActivity,
    NoActivity,
}

/// Reset project by deleting the cloned template projects .git directory
fn reset_project(project_name: &str) -> std::io::Result<()> {
    let path = format!("{}/.git", project_name);
    fs::remove_dir_all(path)?;
    Ok(())
}

fn get_template_branch(template: TemplateBranch) -> &'static str {
    match template {
        TemplateBranch::BasicActivity => "basic-activity",
        TemplateBranch::BottomNavActivity => "bottom-nav-activity",
        TemplateBranch::EmptyActivity => "empty-activity",
        TemplateBranch::NoActivity => "no-activity",
    }
}
