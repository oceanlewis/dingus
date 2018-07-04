#![feature(rust_2018_preview)]

use std::path::PathBuf;

use clap::{App, AppSettings, Arg, SubCommand};

mod app;
mod constants;
mod error;

use crate::{app::run, constants::*, error::Error};

fn main() {
    let config = SubCommand::with_name("print")
        .about(print::ABOUT)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help(common::CONFIG_ARG_TEXT)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("SHELL")
                .help(print::SHELL_ARG_TEXT)
                .takes_value(true),
        );

    let session = SubCommand::with_name("session")
        .about(session::ABOUT)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help(common::CONFIG_ARG_TEXT)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("SHELL")
                .help(session::SHELL_ARG_TEXT)
                .takes_value(true),
        )
        .alias("shell");

    let list = SubCommand::with_name("list").about(list::ABOUT).alias("ls");

    let app = App::new(NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .version(VERSION)
        .author(AUTHORS)
        .long_about(common::ABOUT)
        .subcommand(config)
        .subcommand(session)
        .subcommand(list);

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
