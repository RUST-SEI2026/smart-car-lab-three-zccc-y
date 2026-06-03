use super::executor_def::Executor;
use crate::state::SportsCarState;
use super::Pose;

pub struct SportsCarExecutor;

impl SportsCarExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor {
            pose,
            state: Box::new(SportsCarState::default()),
        }
    }
}