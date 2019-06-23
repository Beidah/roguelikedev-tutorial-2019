use specs::prelude::*;

use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event, Key, KeyCode};

#[macro_use]
extern crate specs_derive;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

#[derive(Default, Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Component for Pos {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(PartialEq)]
struct Exit(bool);

struct PlayerMoveSys;

impl<'a> System<'a> for PlayerMoveSys {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Pos>,
        WriteExpect<'a, Exit>,
    );

    fn run(&mut self, (player, mut pos, mut exit): Self::SystemData) {
        use specs::Join;

        let mut key = Default::default();
        match input::check_for_event(input::KEY_PRESS) {
            Some((_, Event::Key(k))) => {
                key = k;
            }
            _ => {}
        }

        for (_, pos) in (&player, &mut pos).join() {

            if key.pressed {
                match key {
                    Key {
                        code: KeyCode::Up, ..
                    } => pos.y -= 1,
                    Key {
                        code: KeyCode::Down,
                        ..
                    } => pos.y += 1,
                    Key {
                        code: KeyCode::Right,
                        ..
                    } => {
                        pos.x -= 1;
                        println!("{}", pos.x);
                    }
                    Key {
                        code: KeyCode::Left,
                        ..
                    } => pos.x += 1,
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
        }
    }
}

struct PlayerRenderSys;

impl<'a> System<'a> for PlayerRenderSys {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pos>,
        WriteExpect<'a, Root>,
    );

    fn run(&mut self, (player, pos, mut root): Self::SystemData) {
        root.clear();
        for (_, pos) in (&player, &pos).join() {
            root.set_default_foreground(WHITE);
            root.put_char(pos.x, pos.y, '@', BackgroundFlag::None);
        }
        root.flush();
    }
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Roguelike")
        .init();

    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Player>();
    world.add_resource(root);
    world.add_resource(Exit(false));

    world
        .create_entity()
        .with(Player {})
        .with(Pos { x: 1, y: 1 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerRenderSys, "player_render", &[])
        .with_thread_local(PlayerMoveSys)
        .build();

    while !world.read_resource::<Exit>().0 {
        dispatcher.dispatch(&world.res);
        world.maintain();
    }
}
