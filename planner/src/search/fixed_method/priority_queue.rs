use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
    string,
};
use crate::heuristics;

use super::*;

/**
 * Floats don't have Ord because of special values like NaN. But since a
 * heuristic value should never be NaN, we may panic in those scenarios.
 * TODO: Consider using the ordered-float crate instead.
 */
struct OrderedFloat(f32);
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "NaN values are not allowed");
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "NaN values are not allowed");
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "NaN values are not allowed");
        self.0 == other.0
    }
}
impl Eq for OrderedFloat {}

struct PriorityQueue {
    // Given a particular f score (key) store all nodes with this f score (vector)
    map: BTreeMap<OrderedFloat, Vec<Rc<RefCell<SearchNode>>>>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue {
            map: BTreeMap::new(),
        }
    }

    fn insert(&mut self, search_node: Rc<RefCell<SearchNode>>) {
        let key = OrderedFloat(search_node.borrow().f_value());
        if let Some(bucket) = self.map.get_mut(&key) {
            bucket.push(search_node.clone());
        } else {
            self.map.insert(key, vec![search_node.clone()]);
        }
    }

    fn remove(&mut self, search_node: Rc<RefCell<SearchNode>>) {
        let mut bucket_empty = false;
        let key = OrderedFloat(search_node.borrow().f_value());
        if let Some(bucket) = self.map.get_mut(&key) {
            // Retain elements which are not pointer-equal to this search node
            bucket.retain(|x| !Rc::ptr_eq(x, &search_node));
            bucket_empty = bucket.is_empty();
        }
        // Can't have any empty buckets, since the pop_least function assumes the invariant that
        // all buckets contain at least 1 search node
        if bucket_empty {
            self.map.remove(&key);
        }
    }

    fn pop_least(&mut self) -> Option<Rc<RefCell<SearchNode>>> {
        let mut min_key = None;
        let mut bucket_empty = false;
        let mut ret = None;
        if let Some((key, bucket)) = self.map.iter_mut().next() {
            min_key = Some(key);
            ret = Some(bucket.pop().expect(
                "No bucket should be empty in the priority queue"
            ));
            bucket_empty = bucket.is_empty();
        }
        if bucket_empty {
            self.map.remove(&min_key.expect("No min key found"))
        }
        ret
    }
}
