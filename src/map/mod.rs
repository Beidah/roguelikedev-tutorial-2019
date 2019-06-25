use specs::prelude::*;

use tcod::colors::*;
use tcod::console::{Console, Root};
use tcod::noise::*;

pub mod tile;
use tile::Tile;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: i32, height: i32, _world: &mut World) -> Map {
        let noise = Noise::init_with_dimensions(2)
            .noise_type(NoiseType::Default)
            .hurst(0.5)
            .lacunarity(2.0)
            .init();


        let mut tiles = vec![];
        for y in 0..height {
            for x in 0..width {
                if (x == 0 || y == 0 || x == width - 1 || y == height - 1)
                    || noise.get_fbm([y as f32, x as f32], 100) >= 0.25
                {
                    tiles.push(Tile::Wall)
                } else {
                    tiles.push(Tile::Ground);
                }
            }
        }

        Map {
            width,
            height,
            tiles,
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
                        root.put_char_ex(x, y, '#', WHITE, BLACK);
                    }
                }
            }
        }
    }
}