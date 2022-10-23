use std::{fs, path::Path};

use serde_derive::{Deserialize, Serialize};

use crate::{
    config::Config,
    constants::{WORLD_SAVE_FILE_EXTENSION, WORLD_SAVE_LOCATION, YAML_CONFIG_PATH},
    world::World,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Save {
    pub world: World,
    pub date_time_stamp: String,
    pub bouncy_world_engine_version: String,
}

impl Save {
    pub fn new(world: World) -> Save {
        Save {
            world,
            // TODO: add am/pm in format
            date_time_stamp: chrono::offset::Local::now()
                .format("%Y-%m-%dT%H.%M.%S")
                .to_string(),
            bouncy_world_engine_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn load(save_path: &Path) -> Option<Save> {
        // TODO: ability to load config via json?
        if save_path.exists() {
            let save_str = fs::read_to_string(save_path).expect("could not read from file");
            Some(serde_yaml::from_str::<Save>(&save_str).expect("could not read as a save file"))
        } else {
            None
        }
    }

    pub fn save(&self, config: &Config) -> String {
        fs::create_dir_all(WORLD_SAVE_LOCATION)
            .expect("could not create dir at WORLD_SAVE_LOCATION");

        let save_path = &format!("{}\\{}", WORLD_SAVE_LOCATION, &self.date_time_stamp);
        fs::create_dir_all(save_path).expect("could not create dir at self.date_time_stamp");

        let save_str = serde_yaml::to_string(&self).expect("could not write config as yaml");

        std::fs::write(
            format!(
                "{}\\{}.{}",
                save_path, &self.date_time_stamp, WORLD_SAVE_FILE_EXTENSION
            ),
            save_str,
        )
        .expect("could not write to file");

        let config_file_location = format!("{}\\{}", save_path, YAML_CONFIG_PATH);
        config.save_to_yaml_file(&config_file_location);

        save_path.to_string()
    }
}
