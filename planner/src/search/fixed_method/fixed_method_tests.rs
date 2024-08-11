use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};

fn get_domain() -> Rc<DomainTasks> {
    Rc::new(DomainTasks::new(vec![
        Task::Compound(CompoundTask::new(String::from("Eat breakfast"), vec![])),
        Task::Compound(CompoundTask::new(String::from("Pack bag"), vec![])),
        Task::Compound(CompoundTask::new(String::from("Go to work"), vec![])),
    ]))
}

fn get_search_node() -> SearchNode {
    let domain = get_domain();
    SearchNode::new(
        HTN::new(
            BTreeSet::from([10, 20, 30]),
            vec![(10, 20), (30, 20)],
            domain.clone(),
            HashMap::from([
                (10, domain.get_id("Eat breakfast")),
                (20, domain.get_id("Go to work")),
                (30, domain.get_id("Pack bag")),
            ]),
        ),
        HashSet::from([1, 2, 3]),
    )
}

#[cfg(test)]
#[test]
pub fn test_search_node_to_string() {
    let search_node = get_search_node();
    println!("{}", search_node.to_string());
    assert_eq!(
        search_node.to_string(),
        String::from("uncon=[\"10:Eat breakfast\", \"30:Pack bag\"] state=[1, 2, 3]")
    );
}
