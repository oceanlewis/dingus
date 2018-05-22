pub extern crate clap;
//pub extern crate serde_yaml;

use clap::App;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use dingus::error::*;

type VariableList = HashMap<String, String>;

fn print(shell: &str, variable_list: VariableList) -> Result<()> {
    use std::io::{self, Write};

    let mut set_commands: Vec<String> = Vec::with_capacity(variable_list.len());

    for (variable_name, contents) in variable_list {
        match shell {
            "fish" => set_commands.push(
                format_args!(
                    "set -gx {key} \"{value}\"; ",
                    key = variable_name,
                    value = contents
                ).to_string(),
            ),
            _ => set_commands.push(
                format_args!(
                    "export {key}=\"{value}\"; ",
                    key = variable_name,
                    value = contents,
                ).to_string(),
            ),
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(set_commands.join(" ").as_bytes()).unwrap();
    Ok(())
}

fn session(shell: String, variable_list: &VariableList) -> Result<()> {
    use std::process::Command;

    Command::new(shell)
        .envs(variable_list)
        .status()
        .map_err(Error::BadShellVar)?;

    println!("Exiting Dingus Session");
    Ok(())
}

fn list(config_path: &PathBuf) -> Result<()> {
    use std::io::{self, Write};
    let mut stdout = io::stdout();

    for entry in config_path.read_dir()? {
        let path = entry?.path();

        if let Some(extension) = path.extension() {
            if extension == "yaml" {
                let file_name = path.file_name().ok_or(Error::FileNameUnreadable)?;
                writeln!(&mut stdout, "{}", file_name.to_string_lossy())
                    .or(Err(Error::StdIOWriteError))?;
            }
        }
    }

    Ok(())
}

fn load_config_file(path: PathBuf) -> Result<VariableList> {
    use std::io::Read;

    let mut config_file = File::open(path)?;
    let mut file_contents = String::new();
    config_file.read_to_string(&mut file_contents)?;

    let variables: VariableList = serde_yaml::from_str(&file_contents)?;

    Ok(variables)
}

fn parse_shell_env_var() -> Result<String> {
    let shell_var = env::var("SHELL")?;

    let shell_var = shell_var
        .split('/')
        .last()
        .unwrap_or(&shell_var)
        .to_string();

    Ok(shell_var)
}

fn set_dingus_level(variable_list: &mut VariableList) {
    let env_name = "DINGUS_LEVEL";
    let default_level = 1;

    let level = match env::var(&env_name) {
        Ok(var) => match var.parse::<u32>() {
            Ok(current_level) => current_level,
            Err(_) => default_level,
        },
        Err(_) => default_level,
    };

    variable_list.insert(env_name.to_owned(), level.to_string());
}

pub fn run(app: App, mut config_path: PathBuf) -> Result<()> {
    let invocation = app.get_matches();
    let (command_name, subcommand_matches) = invocation.subcommand();
    let subcommand_matches = subcommand_matches.ok_or(Error::SubCommandNotSpecified)?;

    match command_name {
        "print" | "session" => {
            let current_shell = parse_shell_env_var()?;

            let shell_program = subcommand_matches
                .value_of("shell")
                .unwrap_or(&current_shell)
                .to_string();

            config_path.push(subcommand_matches
                .value_of("config")
                .ok_or(Error::ConfigFileNotSpecified)?);

            let mut variable_list = load_config_file(config_path.with_extension("yaml"))?;
            set_dingus_level(&mut variable_list);

            match command_name {
                "print" => print(&shell_program, variable_list),
                "session" => session(shell_program, &variable_list),
                _ => Ok(()),
            }
        }
        "list" => list(&config_path),
        _ => Err(Error::BadCommandError),
    }
}
