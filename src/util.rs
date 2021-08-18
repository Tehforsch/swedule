use std::str;
use std::{
    ffi::OsStr,
    fmt::Display,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub struct ShellCommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn get_shell_command_output<T: Display + AsRef<OsStr>>(
    command_str: &str,
    args: &[T],
    working_dir: Option<&Path>,
    verbose: bool,
) -> ShellCommandOutput {
    let mut command = Command::new(command_str);
    command.args(args).stdin(Stdio::piped());
    if !verbose {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
    }
    if let Some(dir) = working_dir {
        command.current_dir(dir);
    };
    let child = command
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to run command: {}", command_str));

    let output = child.wait_with_output().expect("Failed to read stdout");
    let exit_code = output.status;
    ShellCommandOutput {
        success: exit_code.success(),
        stdout: str::from_utf8(&output.stdout)
            .expect("Failed to decode stdout as utf8")
            .to_owned(),
        stderr: str::from_utf8(&output.stderr)
            .expect("Failed to decode stderr as utf8")
            .to_owned(),
    }
}
