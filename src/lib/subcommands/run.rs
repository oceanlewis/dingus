use crate::lib::{
    config_directory::ConfigDirectory, config_file::ConfigFile, error::Error,
};

use std::{path::PathBuf, process::Command};

pub fn run(
    current_directory: PathBuf,
    config_dir_path: PathBuf,
    given_config_files: Vec<PathBuf>,
    command_and_args: Vec<String>,
    should_disown: bool,
) -> Result<(), Error> {
    let (command, args) = match command_and_args.len() {
        0 => unimplemented!(),
        1 => command_and_args.split_at(1),
        _ => unimplemented!(),
    };

    let mut new_environment = if given_config_files.len() > 0 {
        ConfigDirectory::using(config_dir_path)
            .load_environment(given_config_files)?
    } else {
        ConfigFile::find(&current_directory)
            .unwrap_or(Err(Error::DingusFileNotFound))?
            .environment
    };

    new_environment.increment_level();

    match should_disown {
        true => unimplemented!(),
        false => Command::new(command[0].to_owned())
            .args(args)
            .envs(new_environment.variables)
            .status()
            .map_err(Error::BadShellVar)?, //Command::new(command.first()).args(args).status(),
    };

    Ok(())
}
