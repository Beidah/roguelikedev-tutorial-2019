use specs::prelude::*;

use crate::actions::*;
use crate::components::Position;
use crate::map::{tile::Tile, Map};
use crate::render::Dirty;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Action>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, Dirty>,
    );

    fn run(&mut self, (player, mut position, action, map, mut dirty): Self::SystemData) {
        for (_, mut pos, action) in (&player, &mut position, &action).join() {
            match action.0 {
                ActionType::Move(x, y) => {
                    let new_x = pos.x + x;
                    let new_y = pos.y + y;
                    match map.tiles[((new_y * map.width) + new_x) as usize] {
                        Tile::Ground => {
                            *dirty = Dirty(true);
                            pos.x += x;
                            pos.y += y;
                        }
                        Tile::Wall => {}
                    }
                }
                _ => {}
            }
        }
    }
}
