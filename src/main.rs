use std::path::PathBuf;

extern crate clap;
pub use clap::{App, AppSettings, Arg, SubCommand};

mod dingus;
use dingus::{app::run, error::Error};

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
        .setting(AppSettings::InferSubcommands)
        .version("0.4.2")
        .author("David Armstrong Lewis <david@weassemble.com>")
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
                        .required(false)
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
                        .required(false)
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
                )
                .alias("shell"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about(r#"List possible options available for --config option."#)
                .alias("ls"),
        );

    let mut default_config_path = PathBuf::new();
    default_config_path.push(std::env::home_dir().expect("No home folder for this user."));
    default_config_path.push(".config/dingus");

    if !default_config_path.exists() {
        eprintln!("ERROR: {}", Error::ConfigPathNotFound);
        std::process::exit(1);
    }

    match run(app, default_config_path) {
        Ok(_) => {}
        Err(e) => eprintln!("ERROR: {}", e),
    }
}
