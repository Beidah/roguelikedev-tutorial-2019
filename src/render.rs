use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;

use crate::components::Position;

#[derive(Component, Default, Debug)]
#[storage(DenseVecStorage)]
pub struct Glyph {
    pub character: char,
    pub color: Color,
}

pub struct DrawEntities;

impl<'a> System<'a> for DrawEntities {
    type SystemData = (
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Root>,
    );

    fn run(&mut self, (glyphs, pos, mut con): Self::SystemData) {
        for (glyph, pos) in (&glyphs, &pos).join() {
            con.set_default_foreground(glyph.color);
            con.put_char(pos.x, pos.y, glyph.character, BackgroundFlag::None);
        }

        con.flush();
    }
}