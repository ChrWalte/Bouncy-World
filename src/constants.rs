use crate::{entity::EntityType, world::WorldType};

// cli commands:
// new command:
pub const NEW_COMMAND_LONG: &str = "new";
pub const NEW_COMMAND_SHORT: &str = "n";
pub const NEW_COMMAND_DESCRIPTION: &str =
    "generates a new default world save (in the known-universe folder)";
// save command:
pub const SAVE_COMMAND_LONG: &str = "save";
pub const SAVE_COMMAND_SHORT: &str = "s";
pub const SAVE_COMMAND_DESCRIPTION: &str =
    "save a copy of the current world configuration (to the known-universe folder)";
// config command:
pub const CONFIG_COMMAND_LONG: &str = "config";
pub const CONFIG_COMMAND_SHORT: &str = "c";
pub const CONFIG_COMMAND_DESCRIPTION: &str =
    "generates a new default world configuration (if not already present)";
// help command:
pub const HELP_COMMAND_LONG: &str = "help";
pub const HELP_COMMAND_SHORT: &str = "h";
pub const HELP_COMMAND_DESCRIPTION: &str = "shows a list of all commands and their description";

// configuration files:
pub const YAML_CONFIG_PATH: &str = "config.yaml";
pub const YML_CONFIG_PATH: &str = "config.yml";
pub const JSON_CONFIG_PATH: &str = "config.json";

// bouncy-world save location and file extension
pub const WORLD_SAVE_LOCATION: &str = "known-universe";
pub const WORLD_SAVE_FILE_EXTENSION: &str = "bouncy-world";
// pub const IMAGE_SAVE_LOCATION: &str = "images";
// pub const VIDEO_SAVE_LOCATION: &str = "videos";

// configuration defaults:
// used to log some useful debug information
pub const DEBUG_MODE: bool = false;
// default world settings
pub const DEFAULT_WORLD_WIDTH: i32 = 600;
pub const DEFAULT_WORLD_HEIGHT: i32 = 400;
pub const DEFAULT_WORLD_COLOR: &str = "ffffffff";
pub const DEFAULT_WORLD_TYPE: WorldType = WorldType::Color;
// default entity settings
pub const DEFAULT_ENTITY_COUNT: i32 = 1;
pub const DEFAULT_ENTITY_VELOCITY: i32 = 5;
pub const DEFAULT_ENTITY_COLOR: &str = "000000ff";
pub const DEFAULT_ENTITY_TYPE: EntityType = EntityType::Box(50, 50);
