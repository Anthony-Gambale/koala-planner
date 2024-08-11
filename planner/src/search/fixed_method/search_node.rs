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
pub enum AStarStatus {
    Closed,
    Open,
    New,
}
pub struct Edge {
    pub task_name: String,
    pub method_name: Option<String>,
    pub next_node: Rc<RefCell<SearchNode>>,
}
pub struct SearchNode {
    pub tn: HTN,
    pub state: HashSet<u32>,
    pub progressions: Vec<Edge>,
    pub status: AStarStatus,
    pub parent: Option<Rc<RefCell<SearchNode>>>,
    pub g_value: Option<f32>,
    pub h_value: Option<f32>,
}

impl SearchNode {
    pub fn new(tn: HTN, state: HashSet<u32>) -> SearchNode {
        return SearchNode {
            tn: tn,
            state: state,
            progressions: vec![],
            status: AStarStatus::New,
            parent: None,
            g_value: None,
            h_value: None
        }
    }

    /*
        Same hash -> *maybe* isomorphic
        Different hash -> *definitely not* isomorphic
    */
    pub fn maybe_isomorphic_hash(&self) -> u32 {
        let number_of_tasks = self.tn.count_tasks() as u32;
        let fact_sum: u32 = self.state.iter().sum();
        number_of_tasks + 999983 * fact_sum
    }

    pub fn is_isomorphic(&self, other: Rc<RefCell<SearchNode>>) -> bool {
        self.state == other.borrow().state && HTN::is_isomorphic(&self.tn, &other.borrow().tn)
    }

    pub fn to_string(&self, indentation: String) -> String {
        // Sorting is needed so order is predictable (for tests to pass)
        let mut sorted_state: Vec<&u32> = self.state.iter().collect();
        sorted_state.sort_by(|a, b| a.cmp(b));
        let state = format!("{:?}", sorted_state);
        let uncon_ids = self.tn.get_unconstrained_tasks();
        let mut sorted_uncon_ids: Vec<&u32> = uncon_ids.iter().collect();
        sorted_uncon_ids.sort_by(|a, b| a.cmp(b));
        let mut uncon_tagged = Vec::new();
        for id in uncon_ids {
            let name = self.tn.get_task(id).borrow().get_name();
            uncon_tagged.push(format!("{}:{}", id, name));
        }
        format!("{}uncon={:?} state={}", indentation, uncon_tagged, state)
    }

    pub fn to_string_path(&self) -> String {
        let our_part = self.to_string(String::from(""));
        if let Some(node) = self.parent.clone() {
            node.borrow().to_string_path() + "\n" + &our_part
        } else {
            our_part
        }
    }

    pub fn compute_h_value(&mut self, p: &FONDProblem, h: fn(&FONDProblem, &HashSet<u32>, &HTN) -> f32) {
        self.h_value = Some(h(p, &self.state, &self.tn));
    }

    pub fn f_value(&self) -> f32 {
        if let (Some(g), Some(h)) = (self.g_value, self.h_value) {
            g + h
        } else {
            panic!("Cannot compute f value of a node unless both g and h have been instantiated.")
        }
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_value().partial_cmp(&other.f_value()).expect("Unable to compare the f values of two search nodes.")
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f_value().partial_cmp(&other.f_value())
    }
}

impl Eq for SearchNode {}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_value() == other.f_value()
    }
}

pub fn get_successors_systematic(node: Rc<RefCell<SearchNode>>) -> Vec<(String, Option<String>, SearchNode)> {
    let mut result = vec![];

    let unconstrained = node.borrow().tn.get_unconstrained_tasks();
    let (compounds, actions) = node.borrow().tn.separate_tasks(&unconstrained);

    // Expand a compound task if there is one
    if let Some(id) = compounds.first() {
        if let Task::Compound(cmp) = &*node.borrow().tn.get_task(*id).borrow() {
            for method in cmp.methods.iter() {
                let new_tn = node.borrow().tn.decompose(*id, method);
                let node = SearchNode::new(new_tn, node.borrow().state.clone());
                result.push((cmp.name.clone(), Some(method.name.clone()), node));
            }
        }
    }

    // If a compound task was progressed, exit
    if !result.is_empty() {
        return result;
    }

    // If not, expand primitive tasks
    'prim_loop: for prim in actions.iter() {
        if let Task::Primitive(act) = &*node.borrow().tn.get_task(*prim).borrow() {
            if !act.is_applicable(&node.borrow().state) {
                continue 'prim_loop;
            }
            let new_tn = node.borrow().tn.apply_action(*prim);
            let outcomes = act.transition(&node.borrow().state);
            for outcome in outcomes {
                let node = SearchNode::new(new_tn.clone(), outcome);
                result.push((act.name.clone(), None, node));
            }
        }
    }

    return result;
}
