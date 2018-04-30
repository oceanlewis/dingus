pub extern crate clap;
//pub extern crate serde_yaml;

use clap::App;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::env;

use dingus::error::*;

type VariableList = HashMap<String, String>;

fn print(shell: String, variable_list: VariableList) -> Result<()> {
    use std::io::{self, Write};

    let mut set_commands: Vec<String> = Vec::with_capacity(variable_list.len());

    for (variable_name, contents) in variable_list {
        match shell.as_str() {
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

fn session(shell: String, variable_list: VariableList) -> Result<()> {
    use std::process::Command;

    //let error_text = format!("{} session failed to start!", shell);

    match Command::new(shell).envs(&variable_list).status() {
        Ok(_) => {}
        Err(err) => return Err(Error::BadShellVar(err)),
    };

    println!("Exiting Dingus Session");
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
        .split("/")
        .last()
        .unwrap_or(&shell_var)
        .to_string();

    Ok(shell_var)
}

pub fn run(app: App, mut config_path: PathBuf) -> Result<()> {
    let invocation = app.get_matches();
    let (command_name, subcommand_matches) = invocation.subcommand();
    let subcommand_matches = subcommand_matches.unwrap();

    let current_shell = parse_shell_env_var()?;

    let shell_program = subcommand_matches
        .value_of("shell")
        .unwrap_or(&current_shell)
        .to_string();

    config_path.push(subcommand_matches.value_of("config").unwrap());
    let variable_list = load_config_file(config_path.with_extension("yaml"))?;

    match command_name {
        "print" => print(shell_program, variable_list),
        "session" => session(shell_program, variable_list),
        _ => Err(Error::BadCommandError),
    }
}
