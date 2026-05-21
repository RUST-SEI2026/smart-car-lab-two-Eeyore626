use crate::Pose;

#[derive(Clone, Copy)]
pub(crate) enum Action {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

impl Action {
    pub(crate) fn perform(&self, pose: &mut Pose) {
        match self {
            Action::Forward(o) => pose.forward(*o),
            Action::TurnLeft => pose.turn_left(),
            Action::TurnRight => pose.turn_right(),
        }
    }
}