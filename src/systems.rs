use super::TICK_ENERGY;
use crate::actions::{Action, ActionType, Payload};
use crate::components::{Glyph, Movement, Player, Position, TurnState};
use specs::{ReadStorage, System, WriteExpect, WriteStorage};
use tcod::console::*;
use tcod::input::KeyCode::*;
use tcod::input::{self, Event, Key};

pub struct Draw;

impl<'a> System<'a> for Draw {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Glyph>,
        WriteExpect<'a, Root>,
        WriteExpect<'a, Continue>,
    );

    fn run(&mut self, (position, glyph, mut root, mut cont): Self::SystemData) {
        use specs::Join;
        root.clear();
        for (position, glyph) in (&position, &glyph).join() {
            root.set_default_foreground(glyph.fg_color);
            root.put_char(
                position.x,
                position.y,
                glyph.character,
                BackgroundFlag::None,
            );
        }
        root.flush();

        if root.window_closed() {
            *cont = Continue(false);
        }
    }
}

pub struct ReplenishEnergy;

impl<'a> System<'a> for ReplenishEnergy {
    type SystemData = (WriteStorage<'a, TurnState>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut turn_state, player): Self::SystemData) {
        use specs::Join;
        let mut can_replenish = false;
        for (turn_state, _) in (&turn_state, &player).join() {
            if turn_state.energy < 0 {
                can_replenish = true;
            }
        }

        if can_replenish {
            for turn_state in (&mut turn_state).join() {
                turn_state.energy += TICK_ENERGY;
            }
        }
    }
}

pub struct ChooseAction;

impl<'a> System<'a> for ChooseAction {
    type SystemData = (WriteStorage<'a, TurnState>);

    fn run(&mut self, mut turn_state: Self::SystemData) {
        use specs::Join;
        for turn_state in (&mut turn_state).join() {
            if turn_state.energy >= 0 {
                turn_state.next_action = Some(Action {
                    act_type: ActionType::Movement,
                    payload: Payload {
                        dx: Some(1),
                        dy: None,
                    },
                    energy_cost: 200,
                });
            } else {
                turn_state.next_action = None;
            }
        }
    }
}

pub struct HandleInput;

impl<'a> System<'a> for HandleInput {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, TurnState>);

    fn run(&mut self, (player, mut turn_state): Self::SystemData) {
        use specs::Join;
        let mut action = None;
        let mut key = Default::default();

        match input::check_for_event(input::KEY_PRESS) {
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }

        match key {
            Key { code: Up, .. } => {
                action = Some(Action {
                    act_type: ActionType::Movement,
                    payload: Payload {
                        dx: None,
                        dy: Some(-1),
                    },
                    energy_cost: 100,
                })
            }
            Key { code: Down, .. } => {
                action = Some(Action {
                    act_type: ActionType::Movement,
                    payload: Payload {
                        dx: None,
                        dy: Some(1),
                    },
                    energy_cost: 100,
                })
            }
            Key { code: Left, .. } => {
                action = Some(Action {
                    act_type: ActionType::Movement,
                    payload: Payload {
                        dx: Some(-1),
                        dy: None,
                    },
                    energy_cost: 100,
                })
            }
            Key { code: Right, .. } => {
                action = Some(Action {
                    act_type: ActionType::Movement,
                    payload: Payload {
                        dx: Some(1),
                        dy: None,
                    },
                    energy_cost: 100,
                })
            }

            _ => {}
        }

        for (_, turn_state) in (&player, &mut turn_state).join() {
            turn_state.next_action = action;
        }
    }
}

pub struct Update;

impl<'a> System<'a> for Update {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        WriteStorage<'a, TurnState>,
    );

    fn run(&mut self, (mut position, mut movement, mut turn_state): Self::SystemData) {
        use specs::Join;

        for (position, movement, turn_state) in
            (&mut position, &mut movement, &mut turn_state).join()
        {
            if turn_state.energy >= 0 {
                match turn_state.next_action {
                    Some(act) => {
                        match act.act_type {
                            ActionType::Movement => {
                                match act.payload.dx {
                                    Some(dx) => movement.dx = dx,
                                    _ => movement.dx = 0,
                                }
                                match act.payload.dy {
                                    Some(dy) => movement.dy = dy,
                                    _ => movement.dy = 0,
                                }
                            }
                            _ => {
                                movement.dx = 0;
                                movement.dy = 0
                            }
                        }
                        turn_state.energy -= act.energy_cost;
                        turn_state.next_action = None;
                    }
                    _ => {}
                }
                position.x += movement.dx;
                position.y += movement.dy;
                movement.dx = 0;
                movement.dy = 0;
            }
        }
    }
}

pub struct Continue(pub bool);
