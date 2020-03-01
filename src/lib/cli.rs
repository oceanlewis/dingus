use super::{
    config_directory::ConfigDirectory, config_file::ConfigFile, error::Error,
    shell::Shell, subcommands,
};
use std::{env::current_dir, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

const ABOUT: &str = r#"
Manage your computing environments variables with ease!
Inspired by Juan Karam's original Ruby implementation!"#;

const CONFIG_ARG_TEXT: &str =
    "The Yaml files to be read from that contains the necessary enviroment variables.
The file must live in `~/.config/dingus/`.";

const SESSION_ABOUT: &str =
    "Open a new shell with environment variables applied";

const SESSION_HELP: &str =
    "Specify the shell program you'd like run after your environment is set up";

const LIST_ABOUT: &str = "List possible options available for --config option.";

#[derive(Debug, StructOpt)]
#[structopt(
    about = ABOUT,
    setting = AppSettings::ColoredHelp,
)]
pub enum Cli {
    Run {},

    #[structopt(
      about = SESSION_ABOUT,
      setting = AppSettings::ColoredHelp,
      alias = "shell",
      alias = "s",
    )]
    Session {
        #[structopt(
            help = SESSION_HELP,
        )]
        shell: Option<String>,

        #[structopt(
          help = CONFIG_ARG_TEXT,
          short = "c",
        )]
        configs: Vec<PathBuf>,
    },

    #[structopt(
      about = LIST_ABOUT,
      setting = AppSettings::ColoredHelp,
      alias = "ls",
    )]
    List {},
}

impl Cli {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Cli::Run { .. } => Ok(()),
            Cli::List { .. } => subcommands::list(
                current_dir()?,
                ConfigDirectory::default_directory()?,
            ),
            Cli::Session { shell, configs } => subcommands::session(
                current_dir()?,
                ConfigDirectory::default_directory()?,
                shell.to_owned(),
                configs.to_owned(),
            ),
        }
    }
}
