use crate::interfaces::problem_interface::ProblemInterface;

use super::problem_interface::ProblemStateStatus;
pub trait Action {
    type Problem: ProblemInterface;
    //update the problem given the action
    fn execute(
        &self,
        state: &mut <<Self as Action>::Problem as ProblemInterface>::State,
        problem: &mut Self::Problem,
    ) -> ProblemStateStatus;
}
