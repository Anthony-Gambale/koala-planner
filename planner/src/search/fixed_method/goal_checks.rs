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
        {
            // parent_node lifetime
            let parent_node = parent_unwrap.borrow();
            let edge: &Edge = parent_node.find_edge(&child);
            let old_id: OldID = edge.task_id;

            match &edge.method_name {
                Some(name) => {
                    compound_mapping.insert(old_id, Vec::new());
                    // iterate over them, check their type; if primitive, map to new ID and insert; if compound, insert with Old ID
                    let mut child_set: HashSet<OldID> = child.borrow().tn.get_task_id_set();
                    let parent_set: HashSet<OldID> = parent_node.tn.get_task_id_set();
                    let method_tasks: HashSet<OldID> =
                        child_set.difference(&parent_set).cloned().collect();
                    for method_task in method_tasks {
                        match *child.borrow().tn.get_task(method_task).borrow() {
                            Task::Primitive(_) => compound_mapping.get_mut(&old_id).unwrap().push(
                                TaggedTask::Primitive(*equivalent_ids.get(&method_task).unwrap()),
                            ),
                            Task::Compound(_) => compound_mapping.get_mut(&old_id).unwrap().push(
                                TaggedTask::Compound(method_task)
                            )
                        }
                    }
                }
                None => {
                    let new_id: NewID = next_new_id;
                    next_new_id += 1;
                    tasks.insert(new_id);
                    alpha.insert(new_id, *parent_node.tn.mappings.get(&old_id).unwrap());
                    for greater in parent_node.tn.get_outgoing_edges(old_id) {
                        match *parent_node.tn.get_task(greater).borrow() {
                            Task::Primitive(_) => {
                                orderings.push((new_id, *equivalent_ids.get(&greater).unwrap()));
                            }
                            Task::Compound(_) => {
                                rec_hlpr(&mut orderings, &compound_mapping, new_id, greater);
                            }
                        }
                    }
                }
            }
        } // parent_node lifetime
        child = parent_unwrap;
        parent = child.borrow().parent.clone();
    }

    return HTN::new(tasks, orderings, domain, alpha);
}

fn rec_hlpr(
    orderings: &mut Vec<(NewID, NewID)>,
    compound_mapping: &HashMap<OldID, Vec<TaggedTask>>,
    predecessor_id: NewID,
    compound_task: OldID,
) {
    for task in compound_mapping.get(&compound_task).unwrap() {
        match task {
            TaggedTask::Primitive(id) => orderings.push((predecessor_id, *id)),
            TaggedTask::Compound(id) => rec_hlpr(orderings, compound_mapping, *id, predecessor_id),
        }
    }
}
