use super::*;
use crate::{
    domain_description::{ClassicalDomain, DomainTasks, FONDProblem, Facts},
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

pub struct SearchSpace {
    /*
        SearchNodes in the same bucket are *maybe* isomorphic
        SearchNodes in different buckets are *definitely not* isomorphic
    */
    pub maybe_isomorphic_buckets: HashMap<u32, Vec<Rc<RefCell<SearchNode>>>>,
    pub initial_search_node: Rc<RefCell<SearchNode>>,
}

impl SearchSpace {
    pub fn new(initial_search_node: (HTN, HashSet<u32>)) -> SearchSpace {
        let node = Rc::new(RefCell::new(SearchNode::new(initial_search_node.0, initial_search_node.1)));
        node.borrow_mut().status = AStarStatus::Open;
        let buckets = HashMap::from([(node.borrow().maybe_isomorphic_hash(), vec![node.clone()])]);
        SearchSpace {
            maybe_isomorphic_buckets: buckets,
            initial_search_node: node
        }
    }

    /*
        Either finds an isomorphic node or creates a new one
    */
    pub fn find_isomorphic(&mut self, new_node: SearchNode) -> Rc<RefCell<SearchNode>> {
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
                        let ret = Rc::new(RefCell::new(new_node));
                        bucket.push(ret.clone());
                        ret
                    }
                }
            }
            None => {
                // No bucket exists for this hash, so make one
                let ret = Rc::new(RefCell::new(new_node));
                self.maybe_isomorphic_buckets
                    .insert(hash, vec![ret.clone()]);
                ret
            }
        };
        ret
    }

    pub fn install_successors(&mut self, node: Rc<RefCell<SearchNode>>, successors: Vec<(String, Option<String>, SearchNode)>) {
        for (task_name, method_name, successor) in successors {
            let successor_in_graph: Rc<RefCell<SearchNode>> = self.find_isomorphic(successor);
            node.borrow_mut().progressions.push(Edge {
                task_name: task_name,
                method_name: method_name,
                next_node: successor_in_graph.clone(),
            });
        }
    }

    pub fn to_string(&self) -> String {
        // TODO
        String::from("")
    }
}

pub fn search_result_weak_ld(problem: &FONDProblem, facts: &Facts) -> StrongPolicy {
    // TODO
    panic!();
}
