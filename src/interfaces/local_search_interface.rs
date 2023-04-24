use super::{
    action_interface::Action,
    problem_interface::{ProblemInterface, ProblemStateStatus},
};

pub trait LocalSearchInterface: ProblemInterface {
    type LSAction: Action<Problem = Self>;
    //returns the next action to take given the current state
    fn local_search_action(&mut self, k: usize, state: &mut Self::State) -> Option<Self::LSAction>;
}

pub struct LocalSearchSolver<P>
where
    P: LocalSearchInterface,
{
    pub problem: P,
}

impl<P> LocalSearchSolver<P>
where
    P: LocalSearchInterface,
{
    pub fn new(problem: P) -> Self {
        LocalSearchSolver { problem }
    }

    pub fn solve(&mut self, k: usize, state: &mut <P as ProblemInterface>::State) {
        //init optimization variables
        let mut state_status = ProblemStateStatus::NonTerminal;

        let mut i = 2;

        while i <= k {
            if let Some(action) = self.problem.local_search_action(i, state) {
                //execute action
                action.execute(state, &mut self.problem);
                i = 2;
            } else {
                i += 1;
            }
        }
    }
}
