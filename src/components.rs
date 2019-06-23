use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
