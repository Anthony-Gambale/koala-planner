use std::collections::HashSet;
use super::*;
use crate::{domain_description::{ClassicalDomain, DomainTasks, FONDProblem}, task_network::Method};
use search_graph::*;

mod fixed_method_tests;

// testing that I am able to run tests
pub fn my_first_function(
    tn: Option<HTN>,
    method: Option<Method>,
    domain: Option<DomainTasks>,
    state: Option<HashSet<i32>>,
    problem: Option<FONDProblem>,
) -> i32 {
    1 + 1
}

pub fn run(
    problem: &FONDProblem,
    h: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32
) -> (SearchResult, SearchStats) {
    panic!()
}
