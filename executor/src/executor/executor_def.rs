use crate::action::Assembler;
use crate::state::State;
use super::pose::Pose;

pub struct Executor {
    pub(super) pose: Pose,
    pub(super) state: Box<dyn Assembler>,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: Box::new(State::default()),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'F' => self.state.be_fast(),
                _ => {
                    let actions = self.state.assemble(cmd);
                    for action in actions {
                        action.perform(&mut self.pose)
                    }
                }
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}