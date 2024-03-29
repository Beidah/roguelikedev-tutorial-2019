use doryen_fov::MapData;
use tcod::colors::*;
use tcod::console::*;
use specs::prelude::*;


use crate::components::Position;
use crate::player::Player;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Component, Default, Debug)]
#[storage(DenseVecStorage)]
pub struct Glyph {
    pub character: char,
    pub color: Color,
}

#[derive(Default, Debug)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Default, PartialEq)]
pub struct Dirty(pub bool);

pub struct DrawEntities;

impl<'a> System<'a> for DrawEntities {
    type SystemData = (
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Camera>,
        WriteExpect<'a, Root>,
        ReadExpect<'a, MapData>,
    );

    fn run(&mut self, (glyphs, pos, camera, mut con, fov_map): Self::SystemData) {
        let half_width = camera.width / 2;
        let half_height = camera.height / 2;
        for (glyph, pos) in (&glyphs, &pos).join() {
            if (pos.x > camera.x - half_width as i32 || pos.x < camera.x + half_width as i32)
                && (pos.y > camera.y - half_height as i32 || pos.y < camera.y + half_height as i32)
            {
                let offset_x = pos.x - (camera.x - half_width);
                let offset_y = pos.y - (camera.y - half_height);

                if offset_y < 0
                    || offset_x < 0
                    || offset_y >= SCREEN_HEIGHT
                    || offset_x >= SCREEN_WIDTH
                {
                    continue;
                }

                if fov_map.is_in_fov(pos.x as usize, pos.y as usize) {
                    con.set_default_foreground(glyph.color);
                    con.put_char(offset_x, offset_y, glyph.character, BackgroundFlag::None);
                }
            }
        }

        con.flush();
    }
}

pub struct CameraScroll;

impl<'a> System<'a> for CameraScroll {
    type SystemData = (
        WriteExpect<'a, Camera>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut camera, player, position): Self::SystemData) {
        for (_, position) in (&player, &position).join() {
            // TODO: Give some leeway to camera scroll, and maybe lock it in the map.
            camera.x = position.x;
            camera.y = position.y;
        }
    }

}