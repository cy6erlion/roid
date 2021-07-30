use std::fs;
use std::process::Command;

const URL: &str = "https://gitlab.com/kwatafana/android-project-templates.git";

pub struct Project;

impl Project {
    /// Create a project
    pub fn create(project_name: &str, template: TemplateBranch) -> Result<(), ()> {
        if cfg!(target_os = "windows") {
            panic!("Sorry. Currently only Unix based systems are supported!");
        } else {
            let branch = get_template_branch(template);

            Command::new("git")
                .arg("clone")
                .arg("--single-branch")
                .arg("--branch")
                .arg(branch)
                .arg(URL)
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
