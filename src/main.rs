extern crate clap;
extern crate serde_yaml;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use clap::{App, AppSettings, Arg, SubCommand};

type VariableList = HashMap<String, String>;

fn print(shell: String, variable_list: VariableList) {
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

fn parse_shell_env_var() -> String {
    std::env::var("SHELL")
        .expect("Dingus needs to be run interactively")
        .split("/")
        .last()
        .unwrap()
        .to_string()
}

fn run_app(app: App, mut config_path: PathBuf) {
    let invocation = app.get_matches();
    let (command_name, subcommand_matches) = invocation.subcommand();
    let subcommand_matches = subcommand_matches.unwrap();

    let current_shell = parse_shell_env_var();

    let shell_program = subcommand_matches
        .value_of("shell")
        .unwrap_or(&current_shell)
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
    let long_about = r#"
Manage your computing environments variables with ease!

By default `dingus` will believe your current shell is whatever program your
$SHELL evironment variable is set to. I would not recommend changing this,
instead you can tell `dingus` to use a different shell by supplying a
`--shell SHELL` argument.

Inspired by Juan Karam's original Ruby implementation!"#;

    let config_argument_help = r#"The Yaml file to be read from that contains the necessary
enviroment variables. The file must live in `~/.config/dingus/`.

Custom base paths are currently not supported."#;

    let app = App::new("Dingus")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.3.5")
        .author("David Lewis <david@inkstonehq.com>")
        .long_about(long_about)
        .subcommand(
            SubCommand::with_name("print")
                .about(
                    r#"Print out a shell command you can run to apply variables directly
to your current session."#,
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help(&config_argument_help)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .value_name("SHELL")
                        .help("Specify the name of your shell environment.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("session")
                .about(
                    r#"Open a new shell with environment variables applied. Changes made to
that session will not affect the parent session."#,
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help(&config_argument_help)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .value_name("SHELL")
                        .help(
                            "Specify the shell program you'd like run after your environment \
                             is set up.",
                        )
                        .takes_value(true),
                ),
        );

    let mut default_config_path = PathBuf::new();
    default_config_path.push(std::env::home_dir().expect("No home folder for this user."));
    default_config_path.push(".config/dingus");

    run_app(app, default_config_path);
}
