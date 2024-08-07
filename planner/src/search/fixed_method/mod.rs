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

pub fn my_first_function() -> i32 {
    1 + 1
}

pub struct Edge {
    task_name: String,
    method_name: Option<String>,
    next_node: Rc<SearchNode>,
}

pub struct SearchNode {
    pub tn: HTN,
    pub state: Rc<HashSet<u32>>,
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
        // TODO
        return 0;
    }

    pub fn get_successors(&self) -> Vec<SearchNode> {
        // TODO
        return vec![];
    }

    pub fn to_string(&self, indentation: String) -> String {
        // TODO
        String::from("")
    }
}

pub struct SearchSpace {
    /*
        SearchNodes in the same bucket are *maybe* isomorphic
        SearchNodes in different buckets are *definitely not* isomorphic
        Only need to test a given node with the nodes in the same bucket
    */
    pub maybe_isomorphic_buckets: HashMap<u32, Vec<Rc<SearchNode>>>,
    pub initial_search_node: Rc<SearchNode>,
}

impl SearchSpace {
    /*
        Either finds an isomorphic node or creates a new one
    */
    pub fn find_isomorphic(new_node: SearchNode) -> Rc<SearchNode> {
        // TODO
        Rc::new(new_node)
    }

    pub fn to_string(&self) -> String {
        // TODO
        String::from("")
    }
}

struct AStarSearch {}
impl AStarSearch {
    pub fn run(
        problem: &FONDProblem,
        heuristic: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
    ) -> (SearchResult, SearchStats) {
        // TODO
        panic!()
    }

    fn search(
        problem: &FONDProblem,
        heuristic: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
        search_space: SearchSpace,
        is_goal: fn(&SearchNode, &SearchSpace) -> bool,
    ) -> (SearchResult, SearchStats) {
        // TODO
        panic!()
    }
}
