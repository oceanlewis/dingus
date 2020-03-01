use crate::lib::environment::Environment;
use crate::lib::{
    config_file::ConfigFile,
    error::Error::{self, ConflictingConfigPaths, DingusFileNotFound},
};

use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub struct ConfigDirectory {
    path: PathBuf,
}

impl ConfigDirectory {
    pub fn using(path: PathBuf) -> Self {
        ConfigDirectory { path }
    }

    pub fn default_directory() -> Result<PathBuf, Error> {
        let mut default_config_path =
            dirs::home_dir().expect("No home folder for this user.");
        default_config_path.push(".config/dingus");

        if !default_config_path.exists() {
            return Err(Error::ConfigPathNotFound);
        }

        Ok(default_config_path)
    }

    pub fn search(&self) -> Result<Vec<PathBuf>, Error> {
        Ok(self
            .path
            .read_dir()?
            .flat_map(|read_dir: io::Result<DirEntry>| {
                read_dir.map(|entry: DirEntry| {
                    entry
                        .path()
                        .extension()
                        .and_then(OsStr::to_str)
                        .map(|e: &str| match e {
                            "yaml" | "yml" => Some(entry.path()),
                            _ => None,
                        })
                        .flatten()
                })
            })
            .flatten()
            .collect::<Vec<PathBuf>>())
    }

    pub fn load(&self, these: Vec<PathBuf>) -> Result<Environment, Error> {
        if these.len() == 0 {
            return Err(Error::EmptyConfigList);
        }

        let mut config_files = Vec::new();
        for config in these {
            let path = self.resolve(config)?;
            config_files.push(ConfigFile::load(&path)?);
        }

        Ok(config_files.into_iter().fold(
            Environment::new(),
            |mut acc, this| {
                acc.merge(this.environment);
                acc
            },
        ))
    }

    pub fn resolve(&self, filename: PathBuf) -> Result<PathBuf, Error> {
        fn test_path(
            base_path: PathBuf,
            filename: &Path,
            extension: &str,
        ) -> Option<PathBuf> {
            let mut path = base_path;
            path.push(filename.with_extension(extension));

            match fs::metadata(&path) {
                Ok(_) => Some(path),
                Err(_) => None,
            }
        }

        let resolved_path = match filename.extension().and_then(OsStr::to_str) {
            Some("yaml") | Some("yml") => Ok(filename),
            None => {
                let [yaml_path, yml_path]: [Option<PathBuf>; 2] = [
                    test_path(self.path.clone(), &filename, "yaml"),
                    test_path(self.path.clone(), &filename, "yml"),
                ];

                match (yaml_path, yml_path) {
                    (Some(yaml), None) => Ok(yaml),
                    (None, Some(yml)) => Ok(yml),
                    (Some(yaml), Some(yml)) => Err(ConflictingConfigPaths {
                        one: yaml,
                        two: yml,
                    })?,
                    _ => Err(DingusFileNotFound),
                }
            }
            _ => Err(DingusFileNotFound),
        }?;

        Ok(resolved_path)
    }
}
