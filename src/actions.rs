use specs::{Component, VecStorage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    None,
    Move(i32, i32),
}

#[derive(Component, Default, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Action(pub ActionType);

impl Default for ActionType {
    fn default() -> Self {
        ActionType::None
    }
}
