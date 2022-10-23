use serde_derive::{Deserialize, Serialize};

use crate::config::Config;

// TODO: add note about ball only needing radius
#[derive(Eq, PartialEq, Hash, Deserialize, Serialize, Clone, Debug)]
pub enum EntityType {
    Box(i32, i32),
    Ball(i32),
    // TODO: change this to only the image path and get the width and height dynamically.
    Image(String, i32, i32),
    // TODO: Text just acts as a Box, add ability to write text as pixels
    Text(i32, i32),
}

#[derive(Eq, PartialEq, Hash, Deserialize, Serialize, Debug)]
pub struct Entity {
    pub x_position: i32,
    pub y_position: i32,
    pub x_velocity: i32,
    pub y_velocity: i32,
    pub rgba_hex: String,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(config: &Config) -> Entity {
        Entity {
            // TODO: pull x and y positions from config? user might want to place on screen
            x_position: {
                let random = rand::random::<u32>();
                match &config.entity_type {
                    // TODO: handle this differently?
                    EntityType::Box(width, _) => {
                        (random % ((config.world_width - width) as u32)) as i32
                    }
                    EntityType::Ball(radius) => {
                        (random % ((config.world_width - radius) as u32)) as i32
                    }
                    EntityType::Image(_, width, _) => {
                        (random % ((config.world_width - width) as u32)) as i32
                    }
                    EntityType::Text(width, _) => {
                        (random % ((config.world_width - width) as u32)) as i32
                    }
                }
            },
            y_position: {
                let random = rand::random::<u32>();
                match &config.entity_type {
                    // TODO: handle this differently?
                    EntityType::Box(_, height) => {
                        (random % ((config.world_height - height) as u32)) as i32
                    }
                    EntityType::Ball(radius) => {
                        (random % ((config.world_height - radius) as u32)) as i32
                    }
                    EntityType::Image(_, _, height) => {
                        (random % ((config.world_height - height) as u32)) as i32
                    }
                    EntityType::Text(_, height) => {
                        (random % ((config.world_height - height) as u32)) as i32
                    }
                }
            },
            x_velocity: config.entity_velocity * if rand::random() { 1 } else { -1 },
            y_velocity: config.entity_velocity * if rand::random() { 1 } else { -1 },
            rgba_hex: config.entity_color.to_string(),
            entity_type: config.entity_type.clone(),
        }
    }

    // update entity every frame
    pub fn update(&mut self, config: &Config) {
        match &self.entity_type {
            EntityType::Box(width, height) => {
                &self.width_and_height_bounce(*width, *height, config)
            }
            EntityType::Ball(radius) => &self.radius_bounce(*radius, config),
            EntityType::Image(_, width, height) => {
                &self.width_and_height_bounce(*width, *height, config)
            }
            EntityType::Text(width, height) => {
                &self.width_and_height_bounce(*width, *height, config)
            }
        };
    }

    // draw entity every frame
    // drawing of entity is done is world::draw()
    // pub fn draw(&self, frame: &mut [u8]) {}

    pub fn is_within_entity(&self, x_position: i32, y_position: i32) -> bool {
        match self.entity_type {
            EntityType::Box(width, height) => {
                self.is_within_width_and_height(x_position, y_position, width, height)
            }
            EntityType::Ball(radius) => self.is_within_radius(x_position, y_position, radius),
            EntityType::Image(_, width, height) => {
                self.is_within_width_and_height(x_position, y_position, width, height)
            }
            EntityType::Text(width, height) => {
                self.is_within_width_and_height(x_position, y_position, width, height)
            }
        }
    }

    fn width_and_height_bounce(&mut self, width: i32, height: i32, config: &Config) {
        if self.x_position <= 0 || self.x_position + width > config.world_width {
            self.x_velocity *= -1;
        }

        if self.y_position <= 0 || self.y_position + height > config.world_height {
            self.y_velocity *= -1;
        }

        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;
    }

    fn radius_bounce(&mut self, radius: i32, config: &Config) {
        if self.x_position - radius <= 0 || self.x_position + radius > config.world_width {
            self.x_velocity *= -1;
        }
        if self.y_position - radius <= 0 || self.y_position + radius > config.world_height {
            self.y_velocity *= -1;
        }

        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;
    }

    fn is_within_width_and_height(
        &self,
        x_position: i32,
        y_position: i32,
        width: i32,
        height: i32,
    ) -> bool {
        x_position >= self.x_position
            && x_position < self.x_position + width
            && y_position >= self.y_position
            && y_position < self.y_position + height
    }

    fn is_within_radius(&self, x_position: i32, y_position: i32, radius: i32) -> bool {
        let distance = {
            let x_distance = x_position - self.x_position;
            let y_distance = y_position - self.y_position;
            ((x_distance.pow(2) + y_distance.pow(2)) as f64)
                .sqrt()
                .powi(2)
        };
        distance < (radius as f64).powi(2)
    }
}
