#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProblemStateStatus {
    Terminal,
    NonTerminal,
}

pub trait ProblemInterface {
    //state representation used to query the next action
    type State;

    //initialize state
    fn init_state(&self) -> Self::State;

    fn value(&self) -> f32;
}
