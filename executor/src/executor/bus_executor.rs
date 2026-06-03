use super::executor_def::Executor;
use crate::state::BusState;
use super::Pose;

pub struct BusExecutor;

impl BusExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor {
            pose,
            state: Box::new(BusState::default()),
        }
    }
}