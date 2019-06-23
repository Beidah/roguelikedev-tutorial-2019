

use crate::actions::*;
use specs::{prelude::*, VecStorage};
use tcod::colors;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Glyph {
    pub character: char,
    pub fg_color: colors::Color,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TurnState {
    pub energy: i32,
    pub next_action: Option<Action>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Movement {
    pub dx: i32,
    pub dy: i32,
}
