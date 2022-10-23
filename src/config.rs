use std::fs;

use serde_derive::{Deserialize, Serialize};

use crate::{
    constants::{
        DEBUG_MODE, DEFAULT_ENTITY_COLOR, DEFAULT_ENTITY_COUNT, DEFAULT_ENTITY_TYPE,
        DEFAULT_ENTITY_VELOCITY, DEFAULT_WORLD_COLOR, DEFAULT_WORLD_HEIGHT, DEFAULT_WORLD_TYPE,
        DEFAULT_WORLD_WIDTH,
    },
    entity::EntityType,
    save::Save,
    world::WorldType,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub is_debug_mode: bool,

    pub world_width: i32,
    pub world_height: i32,
    pub world_color: String,
    pub world_type: WorldType,

    pub entity_count: i32,
    pub entity_color: String,
    pub entity_type: EntityType,
    pub entity_velocity: i32,

    pub bouncy_world_engine_version: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            is_debug_mode: DEBUG_MODE,
            world_width: DEFAULT_WORLD_WIDTH,
            world_height: DEFAULT_WORLD_HEIGHT,
            world_color: DEFAULT_WORLD_COLOR.to_string(),
            world_type: DEFAULT_WORLD_TYPE,
            entity_count: DEFAULT_ENTITY_COUNT,
            entity_color: DEFAULT_ENTITY_COLOR.to_string(),
            entity_velocity: DEFAULT_ENTITY_VELOCITY,
            entity_type: DEFAULT_ENTITY_TYPE,

            bouncy_world_engine_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn reverse_from_world_save(world_save: &Save) -> Config {
        Config {
            // use default debug mode:
            is_debug_mode: DEBUG_MODE,

            // use world settings from world save:
            world_width: world_save.world.width,
            world_height: world_save.world.height,
            world_color: world_save.world.background_rgba_hex.to_string(),
            world_type: world_save.world.world_type.clone(),

            // only the entity count can be gotten from the world save
            entity_count: world_save.world.entities.len() as i32,
            entity_color: DEFAULT_ENTITY_COLOR.to_string(),
            entity_velocity: DEFAULT_ENTITY_VELOCITY,
            entity_type: DEFAULT_ENTITY_TYPE,

            bouncy_world_engine_version: world_save.bouncy_world_engine_version.to_string(),
        }
    }

    pub fn load_from_json_file(config_json_path: &str) -> Config {
        let config_str = fs::read_to_string(config_json_path).expect("could not read from file");
        serde_json::from_str::<Config>(&config_str).expect("could not read config as json")
    }

    pub fn save_to_json_file(&self, config_json_path: &str) {
        let config_str =
            serde_json::to_string_pretty(&self).expect("could not write config as json");
        std::fs::write(config_json_path, config_str).expect("could not write to file");
    }

    pub fn load_from_yaml_file(config_yaml_path: &str) -> Config {
        let config_str = fs::read_to_string(config_yaml_path).expect("could not read from file");
        serde_yaml::from_str::<Config>(&config_str).expect("could not read config as yaml")
    }

    pub fn save_to_yaml_file(&self, config_yaml_path: &str) {
        let config_str = serde_yaml::to_string(&self).expect("could not write config as yaml");
        std::fs::write(config_yaml_path, config_str).expect("could not write to file");
    }
}
