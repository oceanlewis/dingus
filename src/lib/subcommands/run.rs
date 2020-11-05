use crate::lib::{
    config_directory::ConfigDirectory, config_file::ConfigFile, error::Error,
};

use std::{path::PathBuf, process::Command};

pub fn run(
    current_directory: PathBuf,
    config_dir_path: PathBuf,
    given_config_files: Vec<PathBuf>,
    command_and_args: Vec<String>,
) -> Result<(), Error> {
    let (command, args) = match command_and_args.len() {
        0 => return Err(Error::NoCommandSupplied),
        _ => command_and_args.split_at(1),
    };
    let command: String = command[0].to_owned();

    let mut new_environment = if !given_config_files.is_empty() {
        ConfigDirectory::using(config_dir_path)
            .load_environment(given_config_files)?
    } else {
        ConfigFile::find(&current_directory)
            .unwrap_or(Err(Error::DingusFileNotFound))?
            .environment
    };
    new_environment.increment_level();

    let exit_status = Command::new(command)
        .args(args)
        .envs(new_environment.variables)
        .status()
        .map_err(Error::BadShellVar)?
        .code()
        .unwrap_or_default();
    std::process::exit(exit_status);
}
