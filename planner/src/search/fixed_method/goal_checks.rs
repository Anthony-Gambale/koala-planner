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

pub fn is_goal_weak_ld(problem: &FONDProblem, leaf_node: Rc<RefCell<SearchNode>>) -> bool {
    leaf_node.borrow().tn.is_empty()
}

type NewID = u32; // ID of a task in the new HTN which we are building
type OldID = u32; // ID of a task in any HTN inside the search space
type TaskName = u32; // Actual task names (same for all HTNs)

pub enum TaggedTask {
    Primitive(NewID),
    Compound(OldID),
}

pub fn is_goal_strong_od(problem: &FONDProblem, leaf_node: Rc<RefCell<SearchNode>>) -> bool {
    if !is_goal_weak_ld(problem, leaf_node.clone()) {
        return false;
    }

    return true;
}

pub fn deorder(leaf_node: Rc<RefCell<SearchNode>>) -> HTN {
    // data structures needed for the task network we're building
    let domain = leaf_node.borrow().tn.domain.clone();
    let mut tasks: BTreeSet<NewID> = BTreeSet::new();
    let mut alpha: HashMap<NewID, TaskName> = HashMap::new();
    let mut orderings: Vec<(NewID, NewID)> = Vec::new();

    // data structures to map IDs between our task network and the ones in the search space
    let mut equivalent_ids: HashMap<OldID, NewID> = HashMap::new();
    let mut compound_mapping: HashMap<OldID, Vec<TaggedTask>> = HashMap::new();

    // not yet handling edge case where initial search node *is* the leaf node
    let mut child = leaf_node.clone();
    let mut parent = child.borrow().parent.clone();
    let mut next_new_id: NewID = 0;

    while parent != None {
        let parent_unwrap = parent.unwrap();
        { // parent_node lifetime
            let parent_node = parent_unwrap.borrow();

            // get edge
            let edge: &Edge = parent_node.find_edge(&child);

            match &edge.method_name {
                Some(name) => {
                    todo!();
                },
                None => {
                    let new_id: NewID = next_new_id;
                    next_new_id += 1;
                },
            }
        } // parent_node lifetime
        // move on to next step
        child = parent_unwrap;
        parent = child.borrow().parent.clone();
    }

    return HTN::new(tasks, orderings, domain, alpha);
}

fn rec_hlpr(
    orderings: &mut Vec<(NewID, NewID)>,
    compound_mapping: &HashMap<OldID, Vec<TaggedTask>>,
    compound_task: OldID,
    predecessor_id: NewID,
) {
    for task in compound_mapping.get(&compound_task).unwrap() {
        match task {
            TaggedTask::Primitive(id) => orderings.push((predecessor_id, *id)),
            TaggedTask::Compound(id) => rec_hlpr(orderings, compound_mapping, *id, predecessor_id),
        }
    }
}
