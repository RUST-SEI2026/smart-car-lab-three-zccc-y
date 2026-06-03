use crate::action::{Assembler, Action};

#[derive(Default, Copy, Clone)]
pub(crate) struct BusState {
    is_reverse: bool,
    is_fast: bool,
}

impl Assembler for BusState {
    fn move_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let step = if self.is_reverse { -1 } else { 1 };
        actions.push(Action::Forward(step));
        if self.is_fast {
            actions.push(Action::Forward(step));
        }
        actions
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let step = if self.is_reverse { -1 } else { 1 };
        actions.push(Action::Forward(step));
        if self.is_fast {
            actions.push(Action::Forward(step));
        }
        actions.push(if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        });
        actions
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        let step = if self.is_reverse { -1 } else { 1 };
        actions.push(Action::Forward(step));
        if self.is_fast {
            actions.push(Action::Forward(step));
        }
        actions.push(if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        });
        actions
    }

    fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }
}