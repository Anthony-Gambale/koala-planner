#![allow(unused)]
use std::collections::{HashMap, HashSet};
use std::env;

use crate::domain_description::{read_json_domain, ClassicalDomain, FONDProblem};
use crate::heuristics::{h_add, h_max};
use crate::relaxation::RelaxedComposition;
use crate::search::{
    astar::AStarResult,
    goal_checks::{is_goal_strong_od, is_goal_weak_ld},
    search_node::{get_successors_systematic, SearchNode},
};
use crate::search::{HeuristicType, SearchResult};
use crate::task_network::HTN;

use super::search_space::SearchSpace;

pub type ClassicalHeuristic = fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32;

pub type HeuristicFn = Box<dyn Fn(&HTN, &HashSet<u32>, &RelaxedComposition, &HashMap<u32, u32>) -> f32>;

pub fn create_function_with_heuristic(
    h_input: ClassicalHeuristic,
) -> HeuristicFn {
    Box::new(move |tn, state, encoder, bijection| {
        let occurances = tn.count_tasks_with_frequency(); // Assuming this returns something iterable
        let task_ids: Vec<u32> = occurances
            .iter()
            .map(|(task, _)| *bijection.get(task).unwrap())
            .collect();
        let relaxed_state = encoder.compute_relaxed_state(&task_ids, state);
        let goal_state = encoder.compute_goal_state(&task_ids);
        let mut val = h_input(&encoder.domain, &relaxed_state, &goal_state);

        // Compensate for the repetition of tasks
        for (_, count) in occurances {
            if count > 1 {
                val += (count - 1) as f32;
            }
        }
        val
    })
}

pub fn zero_heuristic(
) -> HeuristicFn {
    Box::new(move |_, _, _, _| 0.0)
}
