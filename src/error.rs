use std::{env, fmt, io};

use serde_yaml::Error as YamlError;

#[derive(Debug)]
pub enum Error {
    EnvError(env::VarError),
    IOError(io::Error),
    SerdeYamlError(YamlError),
    BadShellVar(io::Error),
    BadCommandError,
    SubCommandNotSpecified,
    DingusFileNotFound,
    FileNameUnreadable,
    StdIOWriteError,
    ConfigPathNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::EnvError(err) => match err {
                env::VarError::NotPresent => {
                    "Looks like your $SHELL environment variable isn't set properly"
                }
                env::VarError::NotUnicode(_) => {
                    "Your $SHELL environment variable isn't valid unicode"
                }
            },
            Error::IOError(_) => {
                "The config file you specified doesn't exist or isn't valid unicode"
            }
            Error::SerdeYamlError(_) => "The config file you specified isn't valid YAML",
            Error::BadCommandError => "Dingus doesn't support that Subcommand",
            Error::BadShellVar(_) => "The <SHELL> argument provided to --shell is invalid",
            Error::SubCommandNotSpecified => "No [SUBCOMMAND] specified",
            Error::DingusFileNotFound => "Couldn't find a YAML file to load",
            Error::FileNameUnreadable => {
                "This file's filename isn't valid unicode and could not be read"
            }
            Error::StdIOWriteError => "Unable to write to Standard Out",
            Error::ConfigPathNotFound => {
                "The default config path of `$HOME/.config/dingus` doesn't exist"
            }
        };

        write!(f, "{}", msg)
    }
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
