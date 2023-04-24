use ndarray::arr2;

pub mod interfaces;
pub mod problems;

use interfaces::greedy_interface::GreedySolver;
use problems::makespan_problem::MakeSpanProblem;

use crate::interfaces::{
    local_search_interface::LocalSearchSolver, problem_interface::ProblemInterface,
};

fn main() {
    let costs = arr2(&[
        [21.0, 43.0, 31.0],
        [11.0, 12.0, 9.0],
        [27.0, 24.0, 20.0],
        [39.0, 32.0, 22.0],
        [72.0, 77.0, 43.0],
        [55.0, 34.0, 44.0],
        [45.0, 40.0, 39.0],
    ]);

    let mut ms_problem = MakeSpanProblem::new(costs);
    let mut state = ms_problem.init_state();
    let mut gs = GreedySolver::new(ms_problem);

    gs.solve(&mut state);

    println!("Greedy solution:\n{:?}", gs.problem.incumbent);
    println!("Greedy value: {}", gs.problem.value());

    let mut ls = LocalSearchSolver::new(gs.problem);

    ls.solve(5, &mut state);

    println!("Local search slution:\n{:?}", ls.problem.incumbent);
    println!("Local search value: {}", ls.problem.value());
}
