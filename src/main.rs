use ansi_term::Style;
use structopt::StructOpt;

mod lib;
use lib::cli::Cli;

fn main() {
    if let Err(e) = Cli::from_args().run() {
        eprintln!("{} {}", Style::new().bold().paint("Error:"), e);
    }
}

// let config = SubCommand::with_name("print")
//     .about(print::ABOUT)
//     .setting(AppSettings::ColoredHelp)
//     .setting(AppSettings::ColorAlways)
//     .arg(
//         Arg::with_name("config")
//             .short("c")
//             .long("config")
//             .value_name("FILE")
//             .help(common::CONFIG_ARG_TEXT)
//             .required(false)
//             .takes_value(true),
//     )
//     .arg(
//         Arg::with_name("shell")
//             .short("s")
//             .long("shell")
//             .value_name("SHELL")
//             .help(print::SHELL_ARG_TEXT)
//             .takes_value(true),
//     );

// let session = SubCommand::with_name("session")
//     .about(session::ABOUT)
//     .setting(AppSettings::ColoredHelp)
//     .setting(AppSettings::ColorAlways)
//     .arg(
//         Arg::with_name("config")
//             .short("c")
//             .long("config")
//             .value_name("FILE")
//             .help(common::CONFIG_ARG_TEXT)
//             .required(false)
//             .takes_value(true),
//     )
//     .arg(
//         Arg::with_name("shell")
//             .short("s")
//             .long("shell")
//             .value_name("SHELL")
//             .help(session::SHELL_ARG_TEXT)
//             .takes_value(true),
//     )
//     .alias("shell");

// let app = App::new(NAME)
//     .setting(AppSettings::ArgRequiredElseHelp)
//     .setting(AppSettings::InferSubcommands)
//     .setting(AppSettings::ColoredHelp)
//     .setting(AppSettings::ColorAlways)
//     .version(VERSION)
//     .author(AUTHORS)
//     .long_about(common::ABOUT)
//     .subcommand(config)
//     .subcommand(session)
//     .subcommand(list);
