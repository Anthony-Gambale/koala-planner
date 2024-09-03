use super::*;
use crate::{
    domain_description::{ClassicalDomain, DomainTasks, FONDProblem},
    search::{search_graph, StrongPolicy},
    task_network::{Method, HTN},
};
use priority_queue::PriorityQueue;
use search_node::{AStarStatus, SearchNode};
use search_space::SearchSpace;
use std::time::{Duration, Instant};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    iter::successors,
    rc::Rc,
    string,
};
use weak_linearization::WeakLinearization;

pub enum AStarResult {
    Strong(StrongPolicy),
    Linear(WeakLinearization),
    NoSolution,
}

pub struct AStarStatistics {
    pub space: SearchSpace,
    pub goal_node: Option<Rc<RefCell<SearchNode>>>,
    pub search_time: Duration,
}

pub fn a_star_search(
    problem: &FONDProblem,
    heuristic_fn: fn(&FONDProblem, &HashSet<u32>, &HTN) -> f32,
    successor_fn: fn(
        &mut SearchSpace,
        Rc<RefCell<SearchNode>>,
    ) -> Vec<(u32, String, Option<String>, SearchNode)>,
    // Using constant function for now
    edge_weight_fn: fn() -> f32,
    goal_check_fn: fn(&FONDProblem, Rc<RefCell<SearchNode>>) -> AStarResult,
) -> (AStarResult, AStarStatistics) {
    let start_time = Instant::now();
    let mut space = SearchSpace::new(problem.init_tn.clone(), problem.initial_state.clone());
    space
        .initial_search_node
        .borrow_mut()
        .compute_h_value(problem, heuristic_fn);
    space.initial_search_node.borrow_mut().g_value = Some(0.0);

    let mut open = PriorityQueue::new();
    open.insert(space.initial_search_node.clone());

    while let Some(parent) = open.pop_least() {
        parent.borrow_mut().status = AStarStatus::Closed;
        space.explored_nodes += 1; // closed set increased in size by 1
        let result = goal_check_fn(problem, parent.clone());
        match result {
            AStarResult::NoSolution => (),
            _ => {
                return (
                    result,
                    AStarStatistics {
                        space: space,
                        goal_node: Some(parent.clone()),
                        search_time: start_time.elapsed(),
                    },
                )
            }
        }
        let successors = successor_fn(&mut space, parent.clone());
        space.install_successors(parent.clone(), successors);
        'improve: for edge in parent.borrow().progressions.iter() {
            // Remove from open with old f value (before updating)
            if edge.next_node.borrow().status == AStarStatus::Open {
                open.remove(edge.next_node.clone());
            }

            {
                // succ_ref lifetime
                let mut succ_ref = edge.next_node.borrow_mut();
                match succ_ref.status {
                    AStarStatus::Open => {
                        if parent.borrow().g_value.unwrap() + edge_weight_fn()
                            < succ_ref.g_value.unwrap()
                        {
                            (*succ_ref).parent = Some(parent.clone());
                            (*succ_ref).g_value =
                                Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                            (*succ_ref).compute_h_value(problem, heuristic_fn);
                        }
                    }
                    AStarStatus::Closed => {
                        if parent.borrow().g_value.unwrap() + edge_weight_fn()
                            < succ_ref.g_value.unwrap()
                        {
                            (*succ_ref).parent = Some(parent.clone());
                            (*succ_ref).g_value =
                                Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                            (*succ_ref).compute_h_value(problem, heuristic_fn);
                            (*succ_ref).status = AStarStatus::Open;
                            space.explored_nodes -= 1; // closed set decreased in size by 1
                        }
                    }
                    AStarStatus::New => {
                        (*succ_ref).parent = Some(parent.clone());
                        (*succ_ref).g_value =
                            Some(parent.borrow().g_value.unwrap() + edge_weight_fn());
                        (*succ_ref).compute_h_value(problem, heuristic_fn);
                        (*succ_ref).status = AStarStatus::Open;
                    }
                }
            } // succ_ref lifetime

            // Insert back into open with new f value
            if (edge.next_node.borrow().status == AStarStatus::Open) {
                open.insert(edge.next_node.clone());
            }
        }
    }
    return (
        AStarResult::NoSolution,
        AStarStatistics {
            space: space,
            goal_node: None,
            search_time: start_time.elapsed(),
        },
    );
}
