use tcod::console::Root;
use tcod::input::{self, Event, Key, KeyCode};

use specs::prelude::*;

use crate::player::Player;
use crate::Exit;

use crate::actions::{Action, ActionType};

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Action>,
        ReadExpect<'a, Root>,
        WriteExpect<'a, Exit>,
    );

    fn run(&mut self, (players, mut actions, con, mut exit): Self::SystemData) {
        let mut key = Default::default();
        let mut action = Default::default();

        match input::check_for_event(input::KEY_PRESS) {
            Some((_, Event::Key(k))) => {
                key = k;
            }
            _ => {}
        }

        if key.pressed {
            match key {
                Key {
                    code: KeyCode::Up, ..
                } => action = ActionType::MoveUp,
                Key {
                    code: KeyCode::Down,
                    ..
                } => action = ActionType::MoveDown,
                Key {
                    code: KeyCode::Right,
                    ..
                } => {
                    action = ActionType::MoveRight;
                }
                Key {
                    code: KeyCode::Left,
                    ..
                } => action = ActionType::MoveLeft,
                // Key {
                //     code: KeyCode::Enter,
                //     alt: true,
                //     ..
                // } => {
                //     let fullscreen = root.is_fullscreen();
                //     root.set_fullscreen(!fullscreen);
                // }
                Key {
                    code: KeyCode::Escape,
                    ..
                } => {
                    *exit = Exit(true);
                }

                _ => {}
            }

        }

        for (_, player_action) in (&players, &mut actions).join() {
            player_action.0 = action;
        }

        *exit = Exit(con.window_closed());
    }
}
