pub extern crate clap;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use dingus::error::*;

type VariableMap = HashMap<String, String>;

pub trait Application<A, E> {
    type A;
    type E;

    fn from_clap(app: clap::App) -> Result<A, E>;
    fn run(self) -> Result<(), E>;
}

enum SubCommand {
    Print,
    Session,
    List,
}

enum Shell {
    BashLike(String),
    Fish(String),
}

impl Shell {
    fn command(&self) -> &str {
        match self {
            Shell::BashLike(bin) => bin,
            Shell::Fish(bin) => bin,
        }
    }
}

pub struct Dingus {
    subcommand: SubCommand,
    shell: Shell,
    environment: VariableMap,
    config_path: PathBuf,
}

impl Application<Dingus, Error> for Dingus {
    type A = Dingus;
    type E = Error;

    fn from_clap(app: clap::App) -> Result<Dingus, Error> {
        let invocation = app.get_matches();

        let (subcommand, subcommand_matches) = match invocation.subcommand() {
            ("print", Some(subcommand_matches)) => (SubCommand::Print, subcommand_matches),
            ("session", Some(subcommand_matches)) => (SubCommand::Session, subcommand_matches),
            ("list", Some(subcommand_matches)) => (SubCommand::List, subcommand_matches),
            _ => return Err(Error::NoSubcommandMatch),
        };

        let shell = {
            let shell_bin = {
                let shell_var = env::var("SHELL")?;
                shell_var
                    .split('/')
                    .last()
                    .unwrap_or(&shell_var)
                    .to_string()
            };

            match shell_bin.as_str() {
                "fish" => Shell::Fish(shell_bin),
                _ => Shell::BashLike(shell_bin),
            }
        };

        let config_path = {
            let mut default_config_path = PathBuf::new();

            default_config_path.push(env::home_dir().expect("No home folder for this user."));
            default_config_path.push(".config/dingus");

            if !default_config_path.exists() {
                return Err(Error::ConfigPathNotFound);
            }

            default_config_path
        };

        // We did this too early, this throws if we just run `dingus list`
        // because it's going to look for a config file and fail!
        let environment: VariableMap = {
            let current_dir = env::current_dir()?;
            let mut config_path = config_path.clone();

            let dingus_file = Dingus::find_a_dingus_file(subcommand_matches, config_path)
                .or_else(|| Dingus::recursively_walk_upwards_for_dingus_file(&current_dir))
                .ok_or(Error::DingusFileNotFound)?;

            let mut variable_map = Dingus::parse_dingus_file(dingus_file)?;
            Dingus::set_dingus_level(&mut variable_map);
            variable_map
        };

        Ok(Dingus {
            subcommand,
            shell,
            environment,
            config_path,
        })
    }

    fn run(self) -> Result<(), Error> {
        match self.subcommand {
            SubCommand::Session => self.session(),
            SubCommand::Print => self.print(),
            SubCommand::List => self.list(),
        }
    }
}

impl Dingus {
    fn parse_dingus_file(path: PathBuf) -> Result<VariableMap, Error> {
        use std::io::Read;

        let mut config_file = File::open(path)?;
        let mut file_contents = String::new();
        config_file.read_to_string(&mut file_contents)?;

        let variables: VariableMap = serde_yaml::from_str(&file_contents)?;

        Ok(variables)
    }

    fn set_dingus_level(variable_list: &mut VariableMap) {
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

    fn recursively_walk_upwards_for_dingus_file(here: &PathBuf) -> Option<PathBuf> {
        let mut possible_location = here.clone();
        possible_location.push(".dingus");

        if possible_location.exists() {
            Some(possible_location)
        } else {
            let parent = here.parent()?;
            Dingus::recursively_walk_upwards_for_dingus_file(&parent.to_path_buf())
        }
    }

    fn find_a_dingus_file(matches: &clap::ArgMatches, mut config_path: PathBuf) -> Option<PathBuf> {
        match matches.value_of("config") {
            Some(filename) => {
                config_path.push(filename);
                Some(config_path.with_extension("yaml"))
            }
            None => None,
        }
    }

    fn session(self) -> Result<(), Error> {
        use std::process::Command;

        Command::new(self.shell.command())
            .envs(self.environment)
            .status()
            .map_err(Error::BadShellVar)?;

        println!("Exiting Dingus Session");
        Ok(())
    }

    fn print(self) -> Result<(), Error> {
        use std::io::{self, Write};

        let mut set_commands: Vec<String> = Vec::with_capacity(self.environment.len());

        for (key, value) in self.environment {
            match self.shell.command() {
                "fish" => set_commands.push(
                    format_args!("set -gx {key} \"{value}\"; ", key = key, value = value)
                        .to_string(),
                ),
                _ => set_commands.push(
                    format_args!("export {key}=\"{value}\"; ", key = key, value = value,)
                        .to_string(),
                ),
            }
        }

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(set_commands.join(" ").as_bytes()).unwrap();
        Ok(())
    }

    fn list(self) -> Result<(), Error> {
        use std::io::{self, Write};
        let mut stdout = io::stdout();

        for entry in self.config_path.read_dir()? {
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
}
