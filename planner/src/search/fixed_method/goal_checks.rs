
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

pub fn is_goal_weak_ld(node: Rc<RefCell<SearchNode>>) -> bool {
    node.borrow().tn.is_empty()
}
