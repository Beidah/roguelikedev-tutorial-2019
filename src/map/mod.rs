use rand::Rng;
use specs::prelude::*;

use tcod::colors::*;
use tcod::console::{Console, Root};

pub mod tile;
use tile::Tile;

mod map_generator;
use map_generator::make_cellular_cave;

use crate::render::Camera;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: i32, height: i32, _world: &mut World) -> Map {

        let tiles = make_cellular_cave(width as usize, height as usize);

        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_random_open_spot(&self) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(0, self.width);
            let y = rng.gen_range(0, self.height);

            if self.tiles[((y * self.width) + x) as usize] == Tile::Ground {
                return (x, y);
            }
        }
    }
}

pub struct DrawMap;

impl<'a> System<'a> for DrawMap {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Camera>,
        WriteExpect<'a, Root>,
    );

    fn run(&mut self, (map, camera, mut root): Self::SystemData) {
        root.clear();
        let camera_half_width = camera.width / 2;
        let camera_half_height = camera.height / 2;
        for y in (camera.y - camera_half_height + 1)..(camera.y + camera_half_height - 1) {
            for x in (camera.x - camera_half_width + 1)..(camera.x + camera_half_width - 1) {
                if y < 0 || x < 0 || y >= map.height || x >= map.width {
                    continue;
                }
                let draw_x = x - (camera.x - camera_half_width);
                let draw_y = y - (camera.y - camera_half_height);
                // println!("Drawing tile at {}, {}", draw_x, draw_y);
                match map.tiles[((y * map.width) + x) as usize] {
                    Tile::Ground => {
                        root.put_char_ex(draw_x, draw_y, ' ', WHITE, BLACK);
                    }
                    Tile::Wall => {
                        root.put_char_ex(draw_x, draw_y, ' ', BRASS, BRASS);
                    }
                }
            }
        }
    }
}