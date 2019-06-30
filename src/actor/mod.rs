use specs::prelude::*;

pub mod actions;

use actions::*;
use crate::player::Player;
use crate::components::Position;

#[derive(Debug, PartialEq)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Actor;

pub struct ActorSystem;

impl<'a> System<'a> for ActorSystem {
    type SystemData = (
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Action>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, Turn>
    );

    fn run(&mut self, (actors, mut actions, positions, player, mut turn): Self::SystemData) {
        if *turn != Turn::Enemy {
            return;
        }

        for (_, action, position) in (&actors, &mut actions, &positions).join() {
            for (_, player_pos) in (&player, &positions).join() {
                if player_pos.x > position.x {
                    *action = Action(ActionType::Move(1, 0));
                } else if player_pos.x < position.x {
                    *action = Action(ActionType::Move(-1, 0));
                } else if player_pos.y > position.y {
                    *action = Action(ActionType::Move(0, 1));
                } else if player_pos.y < position.y {
                    *action = Action(ActionType::Move(0,-1));
                }
            }
        }

        *turn = Turn::Player;
    }
}
