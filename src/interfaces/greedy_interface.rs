use crate::interfaces::action_interface::Action;
use crate::interfaces::problem_interface::{ProblemInterface, ProblemStateStatus};
pub trait GreedyInterface: ProblemInterface {
    type GreedyAction: Action<Problem = Self>;
    //returns the next action to take given the current state
    fn greedy_action(&self, state: &Self::State) -> Self::GreedyAction;
}

pub struct GreedySolver<P>
where
    P: GreedyInterface,
{
    pub problem: P,
}

impl<P> GreedySolver<P>
where
    P: GreedyInterface,
{
    pub fn new(problem: P) -> Self {
        GreedySolver { problem }
    }

    pub fn solve(&mut self, state: &mut <P as ProblemInterface>::State) {
        //init optimization variables
        let mut state_status = ProblemStateStatus::NonTerminal;

        while state_status == ProblemStateStatus::NonTerminal {
            //get next action
            let action = self.problem.greedy_action(&state);

            //execute action
            state_status = action.execute(state, &mut self.problem);
        }
    }
}
