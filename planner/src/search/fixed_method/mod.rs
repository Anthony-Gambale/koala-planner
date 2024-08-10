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

pub fn is_goal_weak_ld(node: Rc<SearchNode>) -> bool {
    node.tn.is_empty()
}

pub fn a_star_search(
    problem: &FONDProblem,
    heuristic_fn: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
    successor_fn: fn(&SearchNode) -> Vec<SearchNode>,
    is_goal: fn(Rc<SearchNode>) -> bool,
) -> (SearchResult, SearchStats) {
    // TODO - 6
    panic!()
}
