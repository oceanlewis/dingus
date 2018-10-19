#[macro_use] extern crate failure;
extern crate clap;

pub use clap::{App, AppSettings, Arg, SubCommand};

mod dingus;
use dingus::{
    app::{Application, Dingus},
    constants::{common, list, print, session, AUTHORS, NAME, VERSION},
};

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
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAlways)
        .version(VERSION)
        .author(AUTHORS)
        .long_about(common::ABOUT)
        .subcommand(config)
        .subcommand(session)
        .subcommand(list);

    match Dingus::from_clap(app).and_then(|app| app.run()) {
        Err(e) => eprintln!("ERROR: {}", e),
        _ => {}
    };
}
