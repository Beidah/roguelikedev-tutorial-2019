#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    Movement,
}

#[derive(Debug, Clone, Copy)]
pub struct Payload {
    pub dx: Option<i32>,
    pub dy: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub act_type: ActionType,
    pub payload: Payload,
    pub energy_cost: i32,
}
