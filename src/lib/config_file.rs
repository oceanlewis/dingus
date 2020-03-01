use crate::lib::{environment::Environment, error::Error};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub struct ConfigFile {
    pub environment: Environment,
}

impl ConfigFile {
    pub fn find(current_location: &Path) -> Option<Result<ConfigFile, Error>> {
        ConfigFile::find_upwards(current_location, Vec::new())
            .first()
            .map(PathBuf::as_path)
            .map(ConfigFile::load)
    }

    pub fn find_upwards(
        current_location: &Path,
        mut found_locations: Vec<PathBuf>,
    ) -> Vec<PathBuf> {
        let mut possible_location = current_location.to_path_buf();
        possible_location.push(".dingus");

        if possible_location.exists() {
            found_locations.push(possible_location)
        }

        if let Some(parent) = current_location.parent() {
            ConfigFile::find_upwards(parent, found_locations)
        } else {
            found_locations
        }
    }

    pub fn load(config_path: &Path) -> Result<Self, Error> {
        let mut config_file = File::open(config_path)?;
        let mut file_contents = String::new();
        config_file.read_to_string(&mut file_contents)?;

        Ok(Self {
            environment: Environment::from_yaml(&file_contents)?,
        })
    }
}
