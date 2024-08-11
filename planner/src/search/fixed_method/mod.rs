use super::*;
use crate::{
    domain_description::{ClassicalDomain, DomainTasks, FONDProblem},
    task_network::Method,
};
use search_graph::*;
use search_node::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    string,
};

mod fixed_method_tests;
mod search_node;
mod search_space;
mod goal_checks;

pub fn a_star_search(
    problem: &FONDProblem,
    heuristic_fn: fn(&FONDProblem, &HashSet<u32>, &HTN) -> f32,
    successor_fn: fn(Rc<RefCell<SearchNode>>) -> Vec<(String, Option<String>, SearchNode)>,
    // Not sure what the type signature should be. Using constant function for now.
    edge_weight_fn: fn() -> f32,
    is_goal: fn(Rc<RefCell<SearchNode>>) -> bool,
    initial_search_node: (&HashSet<u32>, &HTN),
) -> (SearchResult, SearchStats) {
    // TODO - 6
    panic!()
}
