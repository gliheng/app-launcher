use std::{
    process::{Command, Output, Stdio, ExitStatus},
    convert::AsRef,
    path::Path,
};
use once_cell::sync::Lazy;
use regex::Regex;
use regex::Captures;
use crate::models::Context;

pub fn run_command<T: AsRef<Path>>(cmd: &str, dir: T) -> ExitStatus {
    info!("Running command {} in directory {:?}", cmd, dir.as_ref().to_str());
    let mut child = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .current_dir(dir)
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .current_dir(dir)
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to execute command")
    };
    let status = child.wait().expect("Commmand wasn't running");
    if !status.success() {
        // TODO: better ways to hanle error
        panic!("Command run failure");
    }
    status
}

pub fn run_workflow_commands<T: AsRef<Path>>(cmds: &[String], name: &str, context: &Context, dir: T) -> ExitStatus {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(\w+)\}").unwrap());
    let cmds: Vec<String> = cmds.iter().map(|v| RE.replace_all(v, |caps: &Captures| {
        match &caps[1] {
            "name" => {
                name
            }
            "buildScript" => {
                "build"
            }
            "startScript" => {
                "start"
            }
            "distDir" => {
                "dist"
            }
            _ => ""
        }
    }).into_owned()).collect();
    let cmd = cmds.join(";");
    run_command(&cmd, dir)
}
