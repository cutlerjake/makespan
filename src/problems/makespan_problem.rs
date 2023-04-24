use crate::interfaces::{
    action_interface::Action,
    greedy_interface::GreedyInterface,
    local_search_interface::LocalSearchInterface,
    problem_interface::{ProblemInterface, ProblemStateStatus},
};

use itertools::Itertools;
use ndarray::prelude::*;
use ordered_float::OrderedFloat;

pub struct MakeSpanProblem {
    pub cost: Array2<f32>,
    pub incumbent: Vec<usize>,
    pub value: f32,
}

impl MakeSpanProblem {
    pub fn new(cost: Array2<f32>) -> Self {
        let incumbent = Vec::new();
        let value = 0.0;
        MakeSpanProblem {
            cost,
            incumbent,
            value,
        }
    }

    pub fn new_with_incumbent(cost: Array2<f32>, incumbent: Vec<usize>) -> Self {
        let value = 0.0;
        MakeSpanProblem {
            cost,
            incumbent,
            value,
        }
    }

    pub fn num_jobs(&self) -> usize {
        self.cost.shape()[0]
    }

    pub fn num_machines(&self) -> usize {
        self.cost.shape()[1]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MakeSpanState {
    pub current_machine_times: Vec<f32>,
    pub current_job: usize,
    pub num_jobs: usize,
}

impl ProblemInterface for MakeSpanProblem {
    type State = MakeSpanState;
    //type Action = usize;

    fn init_state(&self) -> Self::State {
        MakeSpanState {
            current_machine_times: vec![0.0; self.num_machines()],
            current_job: 0,
            num_jobs: self.num_jobs(),
        }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

pub struct MakeSpanGreedyAction {
    pub action: usize,
}

impl Action for MakeSpanGreedyAction {
    type Problem = MakeSpanProblem;
    fn execute(
        &self,
        state: &mut <<MakeSpanGreedyAction as Action>::Problem as ProblemInterface>::State,
        problem: &mut Self::Problem,
    ) -> ProblemStateStatus {
        problem.incumbent.push(self.action);

        state.current_machine_times[self.action] += problem.cost[[state.current_job, self.action]];
        state.current_job += 1;

        if state.current_job >= state.num_jobs {
            problem.value = *state
                .current_machine_times
                .iter()
                .map(|t| OrderedFloat(*t))
                .max()
                .unwrap();
            ProblemStateStatus::Terminal
        } else {
            ProblemStateStatus::NonTerminal
        }
    }
}

impl GreedyInterface for MakeSpanProblem {
    type GreedyAction = MakeSpanGreedyAction;
    fn greedy_action(&self, state: &Self::State) -> Self::GreedyAction {
        //want to assign the next job to the machine that will result in the lowest total time
        let action = state
            .current_machine_times
            .iter()
            .enumerate()
            .min_by_key(|(j, &time)| OrderedFloat(time + self.cost[[state.current_job, *j]]))
            .unwrap()
            .0;
        MakeSpanGreedyAction { action }
    }
}

pub struct MakeSpanAllocation {
    job: usize,
    machine: usize,
}
pub struct MakeSpanLocalSearchAction {
    pub allocations: Vec<MakeSpanAllocation>,
}

impl LocalSearchInterface for MakeSpanProblem {
    type LSAction = MakeSpanLocalSearchAction;
    fn local_search_action(&mut self, k: usize, state: &mut Self::State) -> Option<Self::LSAction> {
        //current value
        let mut best_value = self.value;
        let mut best_action = None;
        //iterate over all possible job swaps
        for swap_inds in (0..self.num_jobs()).combinations(k) {
            //current job allocation
            let current_allocation = MakeSpanLocalSearchAction {
                allocations: swap_inds
                    .iter()
                    .map(|(i)| MakeSpanAllocation {
                        job: *i,
                        machine: self.incumbent[*i],
                    })
                    .collect(),
            };
            //iterate over all possible ways to allocate jobs
            for job_allocation in (0..k)
                .map(|i| 0..self.num_machines())
                .multi_cartesian_product()
            {
                let action = MakeSpanLocalSearchAction {
                    allocations: swap_inds
                        .iter()
                        .zip(job_allocation.iter())
                        .map(|(i, j)| MakeSpanAllocation {
                            job: *i,
                            machine: *j,
                        })
                        .collect(),
                };

                //execture the action
                action.execute(state, self);

                if self.value < best_value {
                    best_value = self.value;
                    best_action = Some(action);
                }

                //undo the action
                current_allocation.execute(state, self);
            }
        }
        return best_action;
    }
}

impl Action for MakeSpanLocalSearchAction {
    type Problem = MakeSpanProblem;
    fn execute(
        &self,
        state: &mut <<MakeSpanLocalSearchAction as Action>::Problem as ProblemInterface>::State,
        problem: &mut Self::Problem,
    ) -> ProblemStateStatus {
        for MakeSpanAllocation { job: i, machine: j } in self.allocations.iter() {
            //remove the old time
            state.current_machine_times[problem.incumbent[*i]] -=
                problem.cost[[*i, problem.incumbent[*i]]];

            problem.incumbent[*i] = *j;

            //add the new time
            state.current_machine_times[*j] += problem.cost[[*i, *j]];
        }

        let old_val = problem.value;

        problem.value = *state
            .current_machine_times
            .iter()
            .map(|t| OrderedFloat(*t))
            .max()
            .unwrap();

        if old_val > problem.value {
            ProblemStateStatus::NonTerminal
        } else {
            ProblemStateStatus::Terminal
        }
    }
}
