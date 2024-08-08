use super::*;
use crate::{
    domain_description::{ClassicalDomain, DomainTasks, FONDProblem},
    task_network::Method,
};
use search_graph::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    string,
};

mod fixed_method_tests;

pub struct Edge {
    task_name: String,
    method_name: Option<String>,
    next_node: Rc<SearchNode>,
}

pub struct SearchNode {
    pub tn: HTN,
    pub state: HashSet<u32>,
    pub progressions: Vec<Edge>,
    pub status: NodeStatus,
    pub depth: u32,
}

impl SearchNode {
    /*
        If another SearchNode has the same hash, then *maybe* they are isomorphic
        If another SearchNode has a different hash, then *definitely* they are not isomorphic
    */
    pub fn maybe_isomorphic_hash(&self) -> u32 {
        // Hash the task network
        let number_of_tasks = self.tn.count_tasks() as u32;
        // Hash the state
        let fact_sum: u32 = self.state.iter().sum();
        number_of_tasks + 999983 * fact_sum
    }

    pub fn get_successors(&self) -> Vec<SearchNode> {
        // TODO - 4
        return vec![];
    }

    pub fn to_string(&self, indentation: String) -> String {
        // Sort the unconstrained nodes and facts, so that it prints in a deterministic order
        // The order needs to be deterministic for tests to pass
        // It's not a problem that it's costly because this function is only for debugging
        let mut sorted_state: Vec<&u32> = self.state.iter().collect();
        sorted_state.sort_by(|a,b| a.cmp(b));
        let state = format!("{:?}", sorted_state);
        let uncon_ids = self.tn.get_unconstrained_tasks();
        let mut sorted_uncon_ids: Vec<&u32> = uncon_ids.iter().collect();
        sorted_uncon_ids.sort_by(|a,b| a.cmp(b));
        let mut uncon_tagged = Vec::new();
        for id in uncon_ids {
            let name = self.tn.get_task(id).borrow().get_name();
            uncon_tagged.push(format!("{}:{}", id, name));
        }
        format!("{}uncon={:?} state={}", indentation, uncon_tagged, state)
    }
}

pub fn is_goal_weak_ld(node: Rc<SearchNode>, space: Rc<SearchSpace>) -> bool {
    return node.tn.is_empty()
}

pub struct SearchSpace {
    /*
        SearchNodes in the same bucket are *maybe* isomorphic
        SearchNodes in different buckets are *definitely not* isomorphic
    */
    pub maybe_isomorphic_buckets: HashMap<u32, Vec<Rc<SearchNode>>>,
    pub initial_search_node: Rc<SearchNode>,
}

impl SearchSpace {
    /*
        Either finds an isomorphic node or creates a new one
    */
    pub fn find_isomorphic(new_node: SearchNode) -> Rc<SearchNode> {
        // TODO - 5
        Rc::new(new_node)
    }

    pub fn to_string(&self) -> String {
        // TODO - 6
        String::from("")
    }
}

struct AStarSearch {}
impl AStarSearch {
    pub fn run(
        problem: &FONDProblem,
        heuristic: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
    ) -> (SearchResult, SearchStats) {
        // TODO - 8
        panic!()
    }

    fn search(
        problem: &FONDProblem,
        heuristic: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
        search_space: SearchSpace,
        is_goal: fn(Rc<SearchNode>, Rc<SearchSpace>) -> bool,
    ) -> (SearchResult, SearchStats) {
        // TODO - 7
        panic!()
    }
}
