use std::{env, io, path::PathBuf};
use thiserror::Error;

use serde_yaml::Error as YamlError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Looks like your $SHELL environment variable isn't set properly")]
    EnvError(env::VarError),

    #[error(
        "The config file you specified doesn't exist or isn't valid unicode"
    )]
    IOError(io::Error),

    #[error("The config file you specified isn't valid YAML")]
    SerdeYamlError(YamlError),

    #[error("The <SHELL> argument provided to --shell is invalid")]
    BadShellVar(io::Error),

    #[error("Couldn't find a YAML file to load")]
    DingusFileNotFound,

    #[error("The default config path of `~/.config/dingus` doesn't exist")]
    ConfigPathNotFound,

    #[error(
        "Found two conflicting config files, specify the file extension or consider renaming them:
{:?}
{:?}",
        one, two
    )]
    ConflictingConfigPaths { one: PathBuf, two: PathBuf },

    #[error("No config files provided to load")]
    EmptyConfigList,

    #[error("No command was supplied to `run` command")]
    NoCommandSupplied,
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
