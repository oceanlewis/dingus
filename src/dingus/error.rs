use std::result;
use std::fmt;
use std::io;
use std::env;

pub extern crate serde_yaml;
use self::serde_yaml::Error as YamlError;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    EnvError(env::VarError),
    ConfigIOError(io::Error),
    SerdeYamlError(YamlError),
    BadShellVar(io::Error),
    BadCommandError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            &Error::EnvError(ref err) => match err {
                &env::VarError::NotPresent => {
                    "Looks like your $SHELL environment variable isn't set properly"
                }
                &env::VarError::NotUnicode(_) => {
                    "Your $SHELL environment variable isn't valid unicode"
                }
            },
            &Error::ConfigIOError(_) => {
                "The config file you specified doesn't exist or isn't valid unicode"
            }
            &Error::SerdeYamlError(_) => "The config file you specified isn't valid YAML",
            &Error::BadCommandError => "Dingus doesn't support that Subcommand",
            &Error::BadShellVar(_) => "The <SHELL> argument provided to --shell is invalid",
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
        Error::ConfigIOError(err)
    }
}

impl From<YamlError> for Error {
    fn from(err: YamlError) -> Error {
        Error::SerdeYamlError(err)
    }
}
