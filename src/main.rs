use specs::prelude::*;

use tcod::console::*;

mod input;
use input::InputHandler;

mod actions;
mod components;
mod map;

mod player;
mod render;
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

    tcod::system::set_fps(60);

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Action>();
    world.register::<Glyph>();

    world.add_resource(root);
    world.add_resource(Exit(false));

    let map = Map::new(120, 80, &mut world);
    let mut fov_map = doryen_fov::MapData::new(120, 80);
    let (player_x, player_y) = map.get_random_open_spot();
    map.set_fov_walls(&mut fov_map);

    world.add_resource(map);
    world.add_resource(fov_map);
    // world.add_resource(fov_map);
    world.add_resource(Dirty(true));


    let camera = Camera {
        x: player_x,
        y: player_y,
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    };
    world.add_resource(camera);

    world
        .create_entity()
        .with(Player {})
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Glyph {
            character: '@',
            color: tcod::colors::YELLOW,
        })
        .with(Action(ActionType::None))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerSystem, "player_system", &[])
        .with(CameraScroll, "camera", &["player_system"])
        .with(ComputeFOV, "fov", &[])
        .with(DrawMap, "draw_map", &["camera", "fov"])
        .with(DrawEntities, "draw_entities", &["camera", "fov"])
        .with_thread_local(InputHandler)
        .build();

    while !world.read_resource::<Exit>().0 {
        dispatcher.dispatch(&world.res);
        world.maintain();
    }
}
