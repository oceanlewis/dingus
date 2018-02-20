extern crate clap;
extern crate serde_yaml;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use clap::{App, AppSettings, Arg, SubCommand};

type VariableList = HashMap<String, String>;

fn print(shell: String, variable_list: VariableList) {
    for (variable_name, contents) in variable_list {
        match shell.as_str() {
            "fish" => print!(
                "set -gx {key} \"{value}\";",
                key = variable_name,
                value = contents
            ),
            _ => print!(
                "export {key}=\"{value}\";",
                key = variable_name,
                value = contents
            ),
        }
    }
}

fn session(shell: String, variable_list: VariableList) {
    use std::process::Command;

    let error_text = format!("{} session failed to start!", shell);

    Command::new(shell)
        .envs(&variable_list)
        .status()
        .expect(&error_text);

    println!("Exiting Dingus Session");
}

fn load_config_file(path: PathBuf) -> VariableList {
    use std::io::Read;

    let mut config_file = File::open(path).expect("Config file not found!");
    let mut file_contents = String::new();
    config_file.read_to_string(&mut file_contents).unwrap();

    let variables: VariableList =
        serde_yaml::from_str(&file_contents).expect("Malformed config file!");

    variables
}

fn run_app(app: App, mut config_path: PathBuf) {
    let invocation = app.get_matches();
    let (command_name, subcommand_matches) = invocation.subcommand();
    let subcommand_matches = subcommand_matches.unwrap();

    let shell_program = subcommand_matches
        .value_of("shell")
        .unwrap_or("fish")
        .to_string();

    config_path.push(subcommand_matches.value_of("config").unwrap());
    let variable_list = load_config_file(config_path);

    match command_name {
        "print" => {
            print(shell_program, variable_list);
        }
        "session" => {
            session(shell_program, variable_list);
        }
        _ => panic!("Bad command_name passed into run_app()"),
    }
}

fn main() {
    let app = App::new("Dingus")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.2.0")
        .author("David Lewis <david@inkstonehq.com>")
        .about("Manage your computing environments variables with ease! Inspired by Juan Karam's original Ruby implementation!")
        .subcommand(
            SubCommand::with_name("print")
                .about("Print out a shell command you can run to apply variables directly to your current session.")
                .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("The Yaml file to be read from that contains the necessary enviroment variables. The file must live in `~/.config/dingus/` - custom base paths are not supported.")
                        .required(true)
                        .takes_value(true))
                .arg(Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .value_name("SHELL")
                        .help("Specify the name of your shell environment. The default shell is the Fish shell!")
                        .takes_value(true))
        ).subcommand(
            SubCommand::with_name("session")
                .about("Open a new shell with environment variables applied. Changes made to that session will not affect the parent session.")
                .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("The Yaml file to be read from that contains the necessary enviroment variables. The file must live in `~/.config/dingus/` - custom base paths are not supported.")
                        .required(true)
                        .takes_value(true))
                .arg(Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .value_name("SHELL")
                        .help("Specify the shell program you'd like run after your environment is set up. The default shell is the Fish shell!")
                        .takes_value(true)));

    let mut default_config_path = PathBuf::new();
    default_config_path.push(std::env::home_dir().expect("No home folder for this user."));
    default_config_path.push(".config/dingus");

    run_app(app, default_config_path);
}
