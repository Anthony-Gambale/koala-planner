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

pub struct SearchSpace {
    /*
        SearchNodes in the same bucket are *maybe* isomorphic
        SearchNodes in different buckets are *definitely not* isomorphic
    */
    pub maybe_isomorphic_buckets: HashMap<u32, Vec<Rc<RefCell<SearchNode>>>>,
    pub initial_search_node: Rc<RefCell<SearchNode>>,
}

impl SearchSpace {
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

    pub fn install_successors(&self, node: Rc<RefCell<SearchNode>>, successors: Vec<SearchNode>) {
        // TODO - 5
        // Pass each successor through `find_isomorphic` to get an Rc<SearchNode>
        // Then insert each Rc<SearchNode> into the node's progressions vector
        panic!();
    }

    pub fn to_string(&self) -> String {
        // TODO - 7
        String::from("")
    }
}
