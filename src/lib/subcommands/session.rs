use crate::lib::{
    config_directory::ConfigDirectory, config_file::ConfigFile, error::Error,
    shell::Shell,
};

use ansi_term::Style;

use std::{env, path::PathBuf, process::Command};

pub fn session(
    current_directory: PathBuf,
    config_dir_path: PathBuf,
    given_shell_command: Option<String>,
    given_config_files: Vec<PathBuf>,
) -> Result<(), Error> {
    let shell_command = given_shell_command
        .unwrap_or(Shell::current_shell()?.shell_command().to_owned());

    if shell_command.ends_with("fish") {
        unsafe {
            disregard_sigint();
        }
    }

    let mut new_environment = if given_config_files.len() > 0 {
        ConfigDirectory::using(config_dir_path)
            .load_environment(given_config_files)?
    } else {
        ConfigFile::find(&current_directory)
            .unwrap_or(Err(Error::DingusFileNotFound))?
            .environment
    };

    new_environment.increment_level();

    Command::new(shell_command)
        .envs(new_environment.variables)
        .status()
        .map_err(Error::BadShellVar)?;

    println!("{}", Style::new().bold().paint("Exiting Dingus Session\n"));
    Ok(())
}

unsafe fn disregard_sigint() {
    use nix::sys::signal;
    extern "C" fn unabashedly_disregard_signal(_: i32) {}

    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(unabashedly_disregard_signal),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    let _ = signal::sigaction(signal::SIGINT, &sig_action);
}
