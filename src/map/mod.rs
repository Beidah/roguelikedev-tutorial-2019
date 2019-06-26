
use rand::distributions::Uniform;
use rand::Rng;
use specs::prelude::*;

use tcod::colors::*;
use tcod::console::{Console, Root};
use tcod::noise::*;

pub mod tile;
use tile::Tile;

mod map_generator;
use map_generator::make_cellular_cave;

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
    type SystemData = (ReadExpect<'a, Map>, WriteExpect<'a, Root>);

    fn run(&mut self, (map, mut root): Self::SystemData) {
        root.clear();
        for y in 0..map.height {
            for x in 0..map.width {
                match map.tiles[((y * map.width) + x) as usize] {
                    Tile::Ground => {
                        root.put_char_ex(x, y, ' ', WHITE, BLACK);
                    }
                    Tile::Wall => {
                        // println!("Drawing wall at {}, {}", x, y);
                        root.put_char_ex(x, y, ' ', BRASS, BRASS);
                    }
                }
            }
        }
    }
}