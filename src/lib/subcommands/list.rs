use crate::lib::{
    config_directory::ConfigDirectory, config_file::ConfigFile, error::Error,
};

use std::{ffi::OsStr, path::PathBuf};

use ansi_term::Style;

pub fn list(
    current_directory: PathBuf,
    config_dir_path: PathBuf,
) -> Result<(), Error> {
    let config_files_in_path: Vec<PathBuf> =
        ConfigFile::find_upwards(&current_directory, Vec::new());

    let config_files_in_config_directory: Vec<PathBuf> =
        ConfigDirectory::using(config_dir_path).search()?;

    let mut output = Vec::new();
    if config_files_in_path.len() > 0 {
        output.push(format_config_files(
            "Found upwards in current directory",
            config_files_in_path,
            false,
        ));
    }
    if config_files_in_config_directory.len() > 0 {
        output.push(format_config_files(
            "Found in config directory",
            config_files_in_config_directory,
            true,
        ));
    }

    println!("{}", output.join("\n\n"));
    Ok(())
}

fn format_config_files(
    header: &str,
    configs: Vec<PathBuf>,
    print_filename_only: bool,
) -> String {
    let mut output = Vec::new();
    output.push(format!("{}", Style::new().bold().paint(header)));
    output.push(
        configs
            .iter()
            .map(|config: &PathBuf| {
                let text = if print_filename_only {
                    config.file_name().map(OsStr::to_string_lossy)
                } else {
                    Some(config.to_string_lossy())
                };
                format!("  {}", text.unwrap_or("UNREADABLE".into()))
            })
            .collect::<Vec<String>>()
            .join("\n"),
    );
    output.join("\n")
}
