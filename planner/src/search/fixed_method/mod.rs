use super::*;
use crate::{domain_description::DomainTasks, task_network::Method};
use search_graph::*;

mod fixed_method_tests;

// testing that I have access to all the structs I need via the type
// signature of this function.
pub fn my_first_function(
    tn: Option<HTN>,
    method: Option<Method>,
    domain: Option<DomainTasks>,
) -> i32 {
    1 + 1
}
