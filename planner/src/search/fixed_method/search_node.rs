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
pub struct Edge {
    pub task_name: String,
    pub method_name: Option<String>,
    pub next_node: Rc<RefCell<SearchNode>>,
}
pub struct SearchNode {
    pub tn: HTN,
    pub state: HashSet<u32>,
    pub progressions: Vec<Edge>,
    pub status: NodeStatus,
    pub parent: Option<Rc<RefCell<SearchNode>>>,
    pub g_value: f32,
    pub h_value: f32,
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

    pub fn is_isomorphic(&self, other: Rc<RefCell<SearchNode>>) -> bool {
        self.state == other.borrow().state && HTN::is_isomorphic(&self.tn, &other.borrow().tn)
    }

    pub fn to_string(&self, indentation: String) -> String {
        // Sort the unconstrained nodes and facts, so that it prints in a deterministic order
        // The order needs to be deterministic for tests to pass
        // It's not a problem that it's costly because this function is only for debugging
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
                let node = SearchNode {
                    tn: new_tn,
                    state: node.borrow().state.clone(),
                    progressions: vec![],
                    status: NodeStatus::OnGoing,
                    parent: None,
                    g_value: 0.0,
                    h_value: 0.0,
                };
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
                let node = SearchNode {
                    tn: new_tn.clone(),
                    state: outcome,
                    progressions: vec![],
                    status: NodeStatus::OnGoing,
                    parent: None,
                    g_value: 0.0,
                    h_value: 0.0,
                };
                result.push((act.name.clone(), None, node));
            }
        }
    }

    return result;
}
