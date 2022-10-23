use std::{collections::HashMap, env, path::Path};

use cli::BouncyWorldWindow;
use config::Config;
use constants::{JSON_CONFIG_PATH, YAML_CONFIG_PATH, YML_CONFIG_PATH};
use entity::EntityType;
use fltk::{
    app,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};
use pixels::{Pixels, SurfaceTexture};
use save::Save;
use world::{World, WorldType};

pub mod cli;
pub mod config;
pub mod constants;
pub mod entity;
pub mod save;
pub mod world;

// TODO: ability to pause application loop in debug mode
// TODO: ability to take control of entity in debug mode
// TODO: ability to save images of pixel frames
// TODO: ability to save video of bouncy-world

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("unknown command(s): {:?}", args);
        return;
    }

    // start with default config & empty world save
    let mut running_config: Config = Config::new();
    let mut running_world_save: Save;

    // condition to show bouncy-world window
    let mut show_bouncy_world_window = BouncyWorldWindow::Show;

    if args.len() == 2 {
        // one argument was passed in from user
        let given_command = &args[1];
        (running_config, running_world_save, show_bouncy_world_window) =
            cli::run_command(given_command);
    } else {
        // no arguments passed in, try to load config from current directory
        running_config = if Path::new(YAML_CONFIG_PATH).exists() {
            Config::load_from_yaml_file(YAML_CONFIG_PATH)
        } else if Path::new(YML_CONFIG_PATH).exists() {
            Config::load_from_yaml_file(YML_CONFIG_PATH)
        } else if Path::new(JSON_CONFIG_PATH).exists() {
            Config::load_from_json_file(JSON_CONFIG_PATH)
        } else {
            // no config found, use default config
            running_config
        };

        let running_world = World::new(&running_config);
        running_world_save = Save::new(running_world);
    }

    if running_config.is_debug_mode {
        dbg!(&running_config, &running_world_save);
    }

    if show_bouncy_world_window == BouncyWorldWindow::Show {
        // initialize fltk app and fltk window
        let app = app::App::default();
        let mut window = Window::default()
            .with_size(running_config.world_width, running_config.world_height)
            .with_label("Bouncy World");
        window.end();
        window.show();

        // load images into memory
        let mut running_images = HashMap::new();
        for entity in &running_world_save.world.entities {
            match &entity.entity_type {
                EntityType::Image(image_path, _, _) => {
                    running_images.insert(
                        image_path.to_string(),
                        image::open(image_path).expect("could not read image at given path"),
                    );
                }
                _ => (), // not an image
            };
        }
        match &running_world_save.world.world_type {
            WorldType::Image(image_path, _, _) => {
                running_images.insert(
                    image_path.to_string(),
                    image::open(image_path).expect("could not read image at given path"),
                );
            }
            _ => (), // not an image
        }

        // initialize pixels
        let mut pixels = {
            let pixel_width = window.pixel_w() as u32;
            let pixel_height = window.pixel_h() as u32;
            let surface_texture = SurfaceTexture::new(pixel_width, pixel_height, &window);
            Pixels::new(
                running_world_save.world.width as u32,
                running_world_save.world.height as u32,
                surface_texture,
            )
            .expect("pixels failed to initialize")
        };

        // fltk app loop
        while app.wait() {
            // handle events

            // update internal world state
            running_world_save.world.update(&running_config);

            // draw the current frame
            running_world_save
                .world
                .draw(pixels.get_frame(), &running_config, &running_images);
            pixels.render().expect("pixels failed to render");

            // redraw window and trigger event loop
            app::flush();
            app::awake();
        }
    }
}
