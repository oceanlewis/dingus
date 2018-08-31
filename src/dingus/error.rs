use std::{env, io};

pub extern crate serde_yaml;
use self::serde_yaml::Error as YamlError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Looks like your $SHELL environment variable isn't set properly")]
    EnvError(env::VarError),

    #[fail(display = "The config file you specified doesn't exist or isn't valid unicode")]
    IOError(io::Error),

    #[fail(display = "The config file you specified isn't valid YAML")]
    SerdeYamlError(YamlError),

    #[fail(display = "The <SHELL> argument provided to --shell is invalid")]
    BadShellVar(io::Error),

    #[fail(display = "Invalid [SUBCOMMAND] specified")]
    NoSubcommandMatch,

    #[fail(display = "Couldn't find a YAML file to load")]
    DingusFileNotFound,

    #[fail(display = "This file's filename isn't valid unicode and could not be read")]
    FileNameUnreadable,

    #[fail(display = "Unable to write to Standard Out")]
    StdIOWriteError,

    #[fail(display = "The default config path of `$HOME/.config/dingus` doesn't exist")]
    ConfigPathNotFound,
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<YamlError> for Error {
    fn from(err: YamlError) -> Error {
        Error::SerdeYamlError(err)
    }
}
