use super::*;
use crate::{
    domain_description::{ClassicalDomain, DomainTasks, FONDProblem},
    task_network::Method,
};
use search_graph::*;
use search_node::*;
use search_space::SearchSpace;
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
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
    goal_check_fn: fn(&FONDProblem, Rc<RefCell<SearchNode>>) -> bool,
    initial_search_node: (HTN, HashSet<u32>),
) -> Option<String> {
    let mut space = SearchSpace::new(initial_search_node);
    space.initial_search_node.borrow_mut().compute_h_value(problem, heuristic_fn);
    space.initial_search_node.borrow_mut().g_value = Some(0.0);
    // let mut open = BinaryHeap::new();
    // open.push(space.initial_search_node.clone());
    let mut open = BTreeSet::new();
    open.insert(space.initial_search_node.clone());
    while let Some(parent) = open.pop_first() {
        parent.borrow_mut().status = AStarStatus::Closed;
        if goal_check_fn(problem, parent.clone()) {
            return Some(parent.borrow().to_string_path());
        }
        space.install_successors(parent.clone(), successor_fn(parent.clone()));
        for edge in parent.borrow().progressions.iter() {
            let mut succ_ref = edge.next_node.borrow_mut();
            match succ_ref.status {
                AStarStatus::Open => {
                    if parent.borrow().g_value.unwrap() + edge_weight_fn() < succ_ref.g_value.unwrap() {
                        // Remove and re-insert to maintain ordering, since f value changed
                        open.remove(&edge.next_node.clone());
                        (*succ_ref).parent = Some(parent.clone());
                        (*succ_ref).g_value = Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                        (*succ_ref).compute_h_value(problem, heuristic_fn);
                        open.insert(edge.next_node.clone());
                    }
                },
                AStarStatus::Closed => {
                    if parent.borrow().g_value.unwrap() + edge_weight_fn() < succ_ref.g_value.unwrap() {
                        (*succ_ref).parent = Some(parent.clone());
                        (*succ_ref).g_value = Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                        (*succ_ref).compute_h_value(problem, heuristic_fn);
                        (*succ_ref).status = AStarStatus::Open;
                        open.insert(edge.next_node.clone());
                    }
                },
                AStarStatus::New => {
                    (*succ_ref).parent = Some(parent.clone());
                    (*succ_ref).g_value = Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                    (*succ_ref).compute_h_value(problem, heuristic_fn);
                    (*succ_ref).status = AStarStatus::Open;
                    open.insert(edge.next_node.clone());
                },
            }
        }
    }
    return None
}
