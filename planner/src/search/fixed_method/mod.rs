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

    pub fn is_isomorphic(&self, other: Rc<SearchNode>) -> bool {
        self.state == other.state && HTN::is_isomorphic(&self.tn, &other.tn)
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

pub fn is_goal_weak_ld(node: Rc<SearchNode>, space: Rc<SearchSpace>) -> bool {
    node.tn.is_empty()
}

pub fn get_successors_systematic(node: &SearchNode) -> Vec<SearchNode> {
    let mut result = vec![];

    let unconstrained = node.tn.get_unconstrained_tasks();
    let (compounds, actions) = node.tn.separate_tasks(&unconstrained);

    // Expand a compound task if there is one
    if let Some(id) = compounds.first() {
        if let Task::Compound(CompoundTask { name, methods }) = &*node.tn.get_task(*id).borrow() {
            for method in methods.iter() {
                let new_tn = node.tn.decompose(*id, method);
                result.push(SearchNode {
                    tn: new_tn,
                    state: node.state.clone(),
                    progressions: vec![],
                    status: NodeStatus::OnGoing,
                })
            }
        }
    }

    // If a compound task was progressed, exit
    if !result.is_empty() {
        return result;
    }

    // If not, expand primitive tasks
    'prim_loop: for prim in actions.iter() {
        if let Task::Primitive(act) = &*node.tn.get_task(*prim).borrow() {
            if !act.is_applicable(&node.state) {
                continue 'prim_loop;
            }
            let new_tn = node.tn.apply_action(*prim);
            let outcomes = act.transition(&node.state);
            for outcome in outcomes {
                result.push(SearchNode {
                    tn: new_tn.clone(),
                    state: outcome,
                    progressions: vec![],
                    status: NodeStatus::OnGoing,
                })
            }
        }
    }

    return result;
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
    pub fn find_isomorphic(&mut self, new_node: SearchNode) -> Rc<SearchNode> {
        let hash = new_node.maybe_isomorphic_hash();
        let ret = match self.maybe_isomorphic_buckets.get_mut(&hash) {
            Some(bucket) => {
                let mut ret = None;
                'find_isomorphic: for maybe_isomorphic_node in bucket.iter() {
                    if new_node.is_isomorphic(maybe_isomorphic_node.clone()) {
                        ret = Some(maybe_isomorphic_node.clone());
                        break 'find_isomorphic;
                    }
                }
                match ret {
                    Some(isomorphic_node) => {
                        // Found an isomorphic node
                        isomorphic_node
                    }
                    None => {
                        // No isomorphic node, add this to the bucket
                        let ret = Rc::new(new_node);
                        bucket.push(ret.clone());
                        ret
                    }
                }
            }
            None => {
                // No bucket exists for this hash, so make one
                let ret = Rc::new(new_node);
                self.maybe_isomorphic_buckets
                    .insert(hash, vec![ret.clone()]);
                ret
            }
        };
        ret
    }

    pub fn install_successors(&self, node: &mut SearchNode, successors: Vec<SearchNode>) {
        // TODO - 5
        // Pass each successor through `find_isomorphic` to get an Rc<SearchNode>
        // Then insert each Rc<SearchNode> into the node's progressions vector
        panic!();
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
        heuristic_fn: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
        successor_fn: fn(&SearchNode) -> Vec<SearchNode>,
    ) -> (SearchResult, SearchStats) {
        // TODO - 8
        panic!()
    }

    fn search(
        problem: &FONDProblem,
        heuristic_fn: fn(&ClassicalDomain, &HashSet<u32>, &HashSet<u32>) -> f32,
        successor_fn: fn(&SearchNode) -> Vec<SearchNode>,
        search_space: SearchSpace,
        is_goal: fn(Rc<SearchNode>, Rc<SearchSpace>) -> bool,
    ) -> (SearchResult, SearchStats) {
        // TODO - 7
        panic!()
    }
}
