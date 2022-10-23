use std::collections::HashMap;

use crate::{
    config::Config,
    entity::{Entity, EntityType},
};
use hex::FromHex;
use image::{DynamicImage, GenericImageView};
use serde_derive::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub world_type: WorldType,
    pub background_rgba_hex: String,
    pub entities: Vec<Entity>,
}

#[derive(Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub enum WorldType {
    Color,
    // TODO: change this to only the image path and get the width and height dynamically.
    Image(String, i32, i32),
}

fn get_hex_pixel_color_from_image(
    pixel_x_position: i32,
    pixel_y_position: i32,
    image_path: &String,
    images: &HashMap<String, DynamicImage>,
) -> String {
    let image = images
        .get(image_path)
        .expect("image_path not found in images");

    let rgba = image
        .get_pixel((pixel_x_position) as u32, (pixel_y_position) as u32)
        .0;
    let [r, g, b, a] = rgba;
    format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
}

impl World {
    pub fn new(config: &Config) -> World {
        let mut world = World {
            width: config.world_width,
            height: config.world_height,
            world_type: config.world_type.clone(),
            background_rgba_hex: config.world_color.to_string(),
            entities: vec![],
        };

        for _ in (0..config.entity_count).into_iter() {
            world.entities.push(Entity::new(&config));
        }

        world
    }

    // update world every frame
    pub fn update(&mut self, config: &Config) {
        // loop through all entities and update each
        for entity in &mut self.entities {
            entity.update(&config);
        }
    }

    // draw world every frame, the world handles the color of each pixel
    pub fn draw(
        &mut self,
        frame: &mut [u8],
        config: &Config,
        images: &HashMap<String, DynamicImage>,
    ) {
        let mut image_entity_starting_positions: HashMap<usize, (i32, i32)> = HashMap::new();

        // loop through each pixel (frame split in four due to rrggbbaa format)
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // calculate pixels x and y positions on frame
            let pixel_x_position = (i % config.world_width as usize) as i32;
            let pixel_y_position = (i / config.world_width as usize) as i32;

            // TODO: remove?
            let mut temp_entity_rgba_hex_str;

            let mut is_pixel_occupied = false;
            let mut rgba_hex_str = match &self.world_type {
                WorldType::Color => &self.background_rgba_hex,
                WorldType::Image(image_path, width, height) => {
                    if pixel_x_position <= *width && pixel_y_position <= *height {
                        temp_entity_rgba_hex_str = get_hex_pixel_color_from_image(
                            pixel_x_position,
                            pixel_y_position,
                            image_path,
                            images,
                        );

                        &temp_entity_rgba_hex_str
                    } else {
                        &self.background_rgba_hex
                    }
                }
            };
            for (entity_index, entity) in self.entities.iter().enumerate() {
                if entity.is_within_entity(pixel_x_position, pixel_y_position) {
                    rgba_hex_str = match &entity.entity_type {
                        EntityType::Box(_, _) => &entity.rgba_hex,
                        EntityType::Ball(_) => &entity.rgba_hex,
                        EntityType::Image(image_path, _, _) => {
                            if !image_entity_starting_positions.contains_key(&entity_index) {
                                image_entity_starting_positions
                                    .insert(entity_index, (pixel_x_position, pixel_y_position));
                            }

                            if !is_pixel_occupied {
                                is_pixel_occupied = true;
                                let (starting_image_x_position, starting_image_y_position) =
                                image_entity_starting_positions.get(&entity_index).expect(
                                    "failed to get image_entity_starting_positions by entity_index",
                                );

                                let entity_hex_color = get_hex_pixel_color_from_image(
                                    pixel_x_position - starting_image_x_position,
                                    pixel_y_position - starting_image_y_position,
                                    image_path,
                                    images,
                                );

                                if &entity_hex_color[6..] == "00" {
                                    is_pixel_occupied = false;
                                    &rgba_hex_str
                                } else {
                                    temp_entity_rgba_hex_str = entity_hex_color;
                                    &temp_entity_rgba_hex_str
                                }
                            } else {
                                &rgba_hex_str
                            }
                        }
                        EntityType::Text(_, _) => &entity.rgba_hex,
                    };
                }
            }

            pixel.copy_from_slice(
                &<[u8; 4]>::from_hex(rgba_hex_str).expect(
                    "issue converting rgba_str to rgba u8 slice, expected format: rrggbbaa",
                ),
            );
        }
    }
}
