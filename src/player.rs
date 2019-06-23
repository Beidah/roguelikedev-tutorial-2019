use specs::prelude::*;

use crate::actions::*;
use crate::components::Position;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Player;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Action>,
    );

    fn run(&mut self, (player, mut position, action): Self::SystemData) {
        for (_, mut pos, action) in (&player, &mut position, &action).join() {
            match action.0 {
                ActionType::MoveUp => pos.y -= 1,
                ActionType::MoveDown => pos.y += 1,
                ActionType::MoveLeft => pos.x -= 1,
                ActionType::MoveRight => pos.x += 1,
                _ => {}
            }
        }
    }
}
