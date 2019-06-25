use specs::prelude::*;

use tcod::console::*;

mod input;
use input::InputHandler;

mod actions;
mod components;
mod map;
mod render;
mod player;

use actions::*;
use components::Position;
use map::*;
use player::{Player, PlayerSystem};
use render::*;

#[macro_use]
extern crate specs_derive;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

#[derive(PartialEq)]
pub struct Exit(bool);

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Roguelike")
        .init();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Action>();
    world.register::<Glyph>();

    world.add_resource(root);
    world.add_resource(Exit(false));

    let map = Map::new(80, 50, &mut world);
    world.add_resource(map);

    world
        .create_entity()
        .with(Player {})
        .with(Position { x: 1, y: 1 })
        .with(Glyph {
            character: '@',
            color: tcod::colors::YELLOW,
        })
        .with(Action(ActionType::None))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(DrawMap, "draw_map", &[])
        .with(DrawEntities, "draw_entities", &["draw_map"])
        .with(PlayerSystem, "player_system", &[])
        .with_thread_local(InputHandler)
        .build();

    while !world.read_resource::<Exit>().0 {
        dispatcher.dispatch(&world.res);
        world.maintain();
    }
}
