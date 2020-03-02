use crate::lib::{
    config_directory::ConfigDirectory, error::Error, subcommands,
};
use std::{env::current_dir, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

const ABOUT: &str = "Manage your computing environments variables with ease! Inspired by Juan Karam's original Ruby implementation!";

const CONFIG_ARG_TEXT: &str = "The YAML files to be read from that contains the necessary environment variables. These files must live in ~/.config/dingus/.";

const COMMAND_ARG_TEXT: &str =
    "The supplied command and its arguments to be run";

const DISOWN_ARG_TEXT: &str = "Whether the spawned process should be disowned";

const SESSION_ABOUT: &str =
    "Open a new shell with environment variables applied";

const RUN_ABOUT: &str =
    "Run a given command with environment variables applied";

const SESSION_HELP: &str =
    "Specify the shell program you'd like run after your environment is set up";

const LIST_ABOUT: &str = "List possible environments available";

#[derive(Debug, StructOpt)]
#[structopt(
    about = ABOUT,
    setting = AppSettings::ColoredHelp,
)]
pub enum Cli {
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

    #[structopt(
      about = RUN_ABOUT,
      setting = AppSettings::ColoredHelp,
      alias = "r",
    )]
    Run {
        #[structopt(
          help = DISOWN_ARG_TEXT,
          long = "disown",
          short = "d",
        )]
        disown: bool,

        #[structopt(
          help = CONFIG_ARG_TEXT,
          long = "configs",
          short = "c",
        )]
        configs: Vec<PathBuf>,

        #[structopt(
          help = COMMAND_ARG_TEXT,
        )]
        command: Vec<String>,
    },
}

impl Cli {
    pub fn run(&self) -> Result<(), Error> {
        match self {
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
            Cli::Run {
                command,
                configs,
                disown,
            } => subcommands::run(
                current_dir()?,
                ConfigDirectory::default_directory()?,
                configs.to_owned(),
                command.to_owned(),
                *disown,
            ),
        }
    }
}
