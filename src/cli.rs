use std::path::Path;

use crate::{
    config::Config,
    constants::{
        CONFIG_COMMAND_DESCRIPTION, CONFIG_COMMAND_LONG, CONFIG_COMMAND_SHORT,
        HELP_COMMAND_DESCRIPTION, HELP_COMMAND_LONG, HELP_COMMAND_SHORT, JSON_CONFIG_PATH,
        NEW_COMMAND_DESCRIPTION, NEW_COMMAND_LONG, NEW_COMMAND_SHORT, SAVE_COMMAND_DESCRIPTION,
        SAVE_COMMAND_LONG, SAVE_COMMAND_SHORT, WORLD_SAVE_FILE_EXTENSION, YAML_CONFIG_PATH,
        YML_CONFIG_PATH,
    },
    save::Save,
    world::World,
};

#[derive(Eq, PartialEq, Debug)]
pub enum BouncyWorldWindow {
    Show,
    Hide,
}

// TODO: add version command
// TODO: add update command
pub fn run_command(given_command: &str) -> (Config, Save, BouncyWorldWindow) {
    // start with default config & an empty world save
    let mut running_config: Config = Config::new();
    let running_world_save: Save;

    if Path::new(&given_command).exists() {
        // the argument is a valid path, try loading it as a world save or config
        (running_config, running_world_save) = try_load_from_files(&given_command);
        return (running_config, running_world_save, BouncyWorldWindow::Show);
    } else if given_command == NEW_COMMAND_LONG || given_command == NEW_COMMAND_SHORT {
        // run new command:
        running_config = Config::new();
        running_world_save = Save::new(World::new(&running_config));
        let save_file_location = running_world_save.save(&running_config);
        println!(
            "saved default config and default world save to known universe: {}",
            save_file_location
        );
    } else if given_command == SAVE_COMMAND_LONG || given_command == SAVE_COMMAND_SHORT {
        // run save command:
        running_config = if Path::new(YAML_CONFIG_PATH).exists() {
            Config::load_from_yaml_file(YAML_CONFIG_PATH)
        } else if Path::new(YML_CONFIG_PATH).exists() {
            Config::load_from_yaml_file(YML_CONFIG_PATH)
        } else if Path::new(JSON_CONFIG_PATH).exists() {
            Config::load_from_json_file(JSON_CONFIG_PATH)
        } else {
            println!("no config found, loaded default config");
            running_config
        };

        running_world_save = Save::new(World::new(&running_config));
        let save_file_location = running_world_save.save(&running_config);
        println!(
            "saved config and world save to known universe: {}",
            save_file_location
        );
    } else if given_command == CONFIG_COMMAND_LONG || given_command == CONFIG_COMMAND_SHORT {
        // run config command:
        if !Path::new(JSON_CONFIG_PATH).exists() {
            running_config = Config::new();
            // TODO: add ability to save to yaml file
            // running_config.save_to_yaml_file(JSON_CONFIG_PATH);
            running_config.save_to_json_file(JSON_CONFIG_PATH);
            println!("save config file in current directory");
        } else {
            println!("config file already exists in current directory");
        }
        std::process::exit(0);
    } else if given_command == HELP_COMMAND_LONG || given_command == HELP_COMMAND_SHORT {
        // run help command:
        print_help_message();
        std::process::exit(0);
    } else {
        println!("unknown command: {:?}", given_command);
        std::process::exit(0);
    }

    (running_config, running_world_save, BouncyWorldWindow::Hide)
}

fn try_load_from_files(given_path: &str) -> (Config, Save) {
    // start with an empty config & an empty world save
    let running_config: Config;
    let running_world_save: Save;

    let path = Path::new(&given_path);
    let path_file_extension = path
        .extension()
        .expect("expected config file or bouncy-world file");
    if path_file_extension == WORLD_SAVE_FILE_EXTENSION {
        (running_config, running_world_save) = try_load_from_bouncy_world_save(path);
    } else if path.ends_with(YAML_CONFIG_PATH) || path.ends_with(YML_CONFIG_PATH) {
        (running_config, running_world_save) = load_config_from_yaml(given_path);
    } else if path.ends_with(JSON_CONFIG_PATH) {
        (running_config, running_world_save) = load_config_from_json(given_path);
    } else {
        println!("unknown command: {:?}", given_path);
        std::process::exit(0);
    }

    (running_config, running_world_save)
}

fn try_load_from_bouncy_world_save(world_save_path: &Path) -> (Config, Save) {
    // start with default config & default world save
    let mut running_config: Config = Config::new();
    let mut running_world_save: Save = Save::new(World::new(&running_config));

    // try to load world, use default if fails
    running_world_save = match Save::load(world_save_path) {
        Some(world_save) => {
            println!("loaded from world save");
            world_save
        }
        None => {
            println!("unable to load save, loaded default world save");
            running_world_save
        }
    };

    // read from config in same directory as the world save
    let world_save_dir = world_save_path
        .parent()
        .expect("could not get directory from path");
    let world_save_dir = world_save_dir
        .to_str()
        .expect("could not convert path to a str");
    let world_yaml_config = &format!("{}\\{}", world_save_dir, YAML_CONFIG_PATH);
    let world_yml_config = &format!("{}\\{}", world_save_dir, YML_CONFIG_PATH);
    let world_json_config = &format!("{}\\{}", world_save_dir, JSON_CONFIG_PATH);

    running_config = if Path::new(world_yaml_config).exists() {
        Config::load_from_yaml_file(world_yaml_config)
    } else if Path::new(world_yml_config).exists() {
        Config::load_from_yaml_file(world_yml_config)
    } else if Path::new(world_json_config).exists() {
        Config::load_from_json_file(world_json_config)
    }
    // if no config was found, try to load config from current directory
    else if Path::new(YAML_CONFIG_PATH).exists() {
        Config::load_from_yaml_file(YAML_CONFIG_PATH)
    } else if Path::new(YML_CONFIG_PATH).exists() {
        Config::load_from_yaml_file(YML_CONFIG_PATH)
    } else if Path::new(JSON_CONFIG_PATH).exists() {
        Config::load_from_json_file(JSON_CONFIG_PATH)
    } else {
        println!("no config found, created config from world save");
        Config::reverse_from_world_save(&running_world_save)
    };

    (running_config, running_world_save)
}

fn load_config_from_yaml(config_path: &str) -> (Config, Save) {
    let running_config = Config::load_from_yaml_file(config_path);
    let running_world_save = Save::new(World::new(&running_config));
    println!("loaded from yaml config");

    (running_config, running_world_save)
}

fn load_config_from_json(config_path: &str) -> (Config, Save) {
    let running_config = Config::load_from_json_file(config_path);
    let running_world_save = Save::new(World::new(&running_config));
    println!("loaded from json config");

    (running_config, running_world_save)
}

fn print_help_message() {
    println!("The Bouncy World Engine - v{}\n", env!("CARGO_PKG_VERSION"));
    println!("Usage: bouncy-world.exe <COMMAND>\n");
    println!("Commands:");
    println!("  {}\t\t{}", NEW_COMMAND_LONG, NEW_COMMAND_DESCRIPTION);
    println!("  {}\t\t{}", SAVE_COMMAND_LONG, SAVE_COMMAND_DESCRIPTION);
    println!("  {}\t{}", CONFIG_COMMAND_LONG, CONFIG_COMMAND_DESCRIPTION);
    println!("  {}\t\t{}", HELP_COMMAND_LONG, HELP_COMMAND_DESCRIPTION);
}
