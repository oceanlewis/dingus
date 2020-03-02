use failure::Fail;
use std::{env, io, path::PathBuf};

use serde_yaml::Error as YamlError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "Looks like your $SHELL environment variable isn't set properly"
    )]
    EnvError(env::VarError),

    #[fail(
        display = "The config file you specified doesn't exist or isn't valid unicode"
    )]
    IOError(io::Error),

    #[fail(display = "The config file you specified isn't valid YAML")]
    SerdeYamlError(YamlError),

    #[fail(display = "The <SHELL> argument provided to --shell is invalid")]
    BadShellVar(io::Error),

    #[fail(display = "Couldn't find a YAML file to load")]
    DingusFileNotFound,

    #[fail(
        display = "The default config path of `~/.config/dingus` doesn't exist"
    )]
    ConfigPathNotFound,

    #[fail(
        display = "Found two conflicting config files, specify the file extension or consider renaming them:
{:?}
{:?}",
        one, two
    )]
    ConflictingConfigPaths { one: PathBuf, two: PathBuf },

    #[fail(display = "No config files provided to load")]
    EmptyConfigList,

    #[fail(display = "No command was supplied to `run` command")]
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
