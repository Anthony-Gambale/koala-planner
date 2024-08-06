use super::*;
use crate::task_network::Method;
use search_graph::*;

mod fixed_method_tests;

// testing that I have access to all the structs I need via the type
// signature of this function.
pub fn my_first_function(tn: Option<HTN>, method: Option<Method>) -> i32 {
  1 + 1
}
