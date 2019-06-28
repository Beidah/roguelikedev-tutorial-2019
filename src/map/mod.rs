use rand::Rng;
use specs::prelude::*;

use tcod::{
    colors::*,
    console::{Console, Root},
};

use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};

pub mod tile;
use tile::Tile;

mod map_generator;
use map_generator::make_cellular_cave;

use crate::components::Position;
use crate::player::Player;
use crate::render::{Camera, Dirty};

const TORCH_RADIUS: usize = 7;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub fov: FovRecursiveShadowCasting,
    pub explored: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize, _world: &mut World) -> Map {

        let tiles = make_cellular_cave(width as usize, height as usize);
        let explored = vec![false; width * height];

        Map {
            width: width as i32,
            height: height as i32,
            tiles,
            fov: FovRecursiveShadowCasting::new(),
            explored,
        }
    }

    pub fn set_fov_walls(&self, fov_map: &mut MapData) {
        for (i, tile) in self.tiles.iter().enumerate() {
            match tile {
                Tile::Ground => {}
                Tile::Wall => {
                    fov_map.set_transparent(
                        i % self.width as usize,
                        i / self.width as usize,
                        false,
                    );
                }
            }
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

    #[allow(dead_code)]
    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.width as usize + x]
    }

    #[allow(dead_code)]
    pub fn get_mut_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[y * self.width as usize + x]
    }
}

pub struct DrawMap;

impl<'a> System<'a> for DrawMap {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, MapData>,
        ReadExpect<'a, Camera>,
        WriteExpect<'a, Root>,
    );

    fn run(&mut self, (mut map, fov_map, camera, mut root): Self::SystemData) {
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
                let visible = fov_map.is_in_fov(x as usize, y as usize);
                let index = ((y * map.width) + x) as usize;
                let tile = map.tiles[index];
                // println!("Drawing tile at {}, {}", draw_x, draw_y);
                match (tile, visible) {
                    (Tile::Ground, true) => {
                        if !map.explored[index] {
                            map.explored[index] = true
                        }
                        root.put_char_ex(draw_x, draw_y, ' ', DARK_GREY, DARK_GREY);
                    }
                    (Tile::Ground, false) => {
                        if map.explored[index] {
                            root.put_char_ex(draw_x, draw_y, ' ', DARKEST_BLUE, DARKEST_BLUE);
                        }
                    }
                    (Tile::Wall, true) => {
                        if !map.explored[index] {
                            map.explored[index] = true
                        }
                        root.put_char_ex(draw_x, draw_y, ' ', DARK_AMBER, DARK_AMBER);
                    }
                    (Tile::Wall, false) => {
                        if map.explored[index] {
                            root.put_char_ex(draw_x, draw_y, ' ', DARKEST_VIOLET, DARKEST_VIOLET);
                        }
                    }
                }
            }
        }
    }
}

pub struct ComputeFOV;

impl<'a> System<'a> for ComputeFOV {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteExpect<'a, MapData>,
        WriteExpect<'a, Dirty>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut map, mut fov_map, mut dirty, player, player_pos): Self::SystemData) {
        if dirty.0 {
            fov_map.clear_fov();
            for (_, position) in (&player, &player_pos).join() {
                map.fov.compute_fov(
                    &mut fov_map,
                    position.x as usize,
                    position.y as usize,
                    TORCH_RADIUS,
                    true,
                );
            }

            *dirty = Dirty(false);
        }
    }
}