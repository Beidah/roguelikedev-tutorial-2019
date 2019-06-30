use specs::*;

use crate::{
    actor::Actor,
    components::Position,
    map::{tile::Tile, Map},
    render::Dirty
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    None,
    Move(i32, i32),
}

#[derive(Component, Default, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Action(pub ActionType);

impl Default for ActionType {
    fn default() -> Self {
        ActionType::None
    }
}

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Action>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, Dirty>,
    );

    fn run(&mut self, (actors, mut position, mut action, map, mut dirty): Self::SystemData) {
        for (_, mut pos, action) in (&actors, &mut position, &mut action).join() {
            match action.0 {
                ActionType::Move(x, y) => {
                    let new_x = pos.x + x;
                    let new_y = pos.y + y;
                    match map.tiles[((new_y * map.width) + new_x) as usize] {
                        Tile::Ground => {
                            *dirty = Dirty(true);
                            pos.x = new_x;
                            pos.y = new_y;
                        }
                        Tile::Wall => {}
                    }
                }
                _ => {}
            }

            *action = Action(ActionType::None);
        }
    }
}
