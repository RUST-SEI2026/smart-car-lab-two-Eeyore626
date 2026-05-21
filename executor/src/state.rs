use crate::action::Action;

#[derive(Default, Clone, Copy)]
pub(crate) struct State {
    pub is_reverse: bool,
    pub is_fast: bool,
}

impl State {
    pub(crate) fn toggle_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn toggle_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }

    pub(crate) fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_actions(),
            'L' => self.left_actions(),
            'R' => self.right_actions(),
            _ => vec![],
        }
    }

    fn move_actions(&self) -> Vec<Action> {
        let step = if self.is_fast { 2 } else { 1 };
        let dir = if self.is_reverse { -1 } else { 1 };

        let mut actions = Vec::new();
        for _ in 0..step {
            actions.push(Action::Forward(dir));
        }
        actions
    }

    fn left_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();

        if self.is_fast {
            let dir = if self.is_reverse { -1 } else { 1 };
            actions.push(Action::Forward(dir));
        }

        if self.is_reverse {
            actions.push(Action::TurnRight);
        } else {
            actions.push(Action::TurnLeft);
        }

        actions
    }

    fn right_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();

        if self.is_fast {
            let dir = if self.is_reverse { -1 } else { 1 };
            actions.push(Action::Forward(dir));
        }

        if self.is_reverse {
            actions.push(Action::TurnLeft);
        } else {
            actions.push(Action::TurnRight);
        }

        actions
    }
}