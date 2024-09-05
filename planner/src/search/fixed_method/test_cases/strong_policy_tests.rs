use crate::domain_description::Facts;
use crate::domain_description::FONDProblem;
use super::super::*;
use std::{
    collections::{BTreeSet, HashMap, HashSet}, vec, rc::Rc,
};
use super::super::astar::{a_star_search, AStarResult};
use super::super::goal_checks::*;
use search_node::get_successors_systematic;
use search_node::SearchNode;

#[cfg(test)]
#[test]
pub fn strong_od_problem_1() {
    let f1 = String::from("f1");
    let f2 = String::from("f2");
    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");
    let t = String::from("t");
    let mt = String::from("mt");
    let minit = String::from("minit");
    let init = String::from("init");
    let problem = FONDProblem::new(
        vec![f1.clone(), f2.clone()],
        vec![
            (a.clone(), vec![], vec![
                (vec![f1.clone()], vec![]),
                (vec![f2.clone()], vec![])
            ]),
            (b.clone(), vec![f1.clone()], vec![(vec![f2.clone()], vec![])]),
            (c.clone(), vec![f2.clone()], vec![(vec![f1.clone()], vec![])])
        ],
        vec![
            (
                minit.clone(), init.clone(),
                vec![a.clone(), t.clone()],
                vec![(0,1)]
            ),
            (
                mt.clone(), t.clone(),
                vec![b.clone(), c.clone()],
                vec![]
            )
        ],
        vec![init.clone(), t.clone()],
        HashSet::new(),
        init.clone(),
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_strong_od,
    );
    if let AStarResult::Strong(policy) = solution {
        println!("\nPLAN\n");
        println!("{}", policy);
    } else {
        println!("\nNO PLAN\n");
    }
    println!("\nSEARCH SPACE explored:{} total:{}\n", statistics.space.explored_nodes, statistics.space.total_nodes);
    println!("{}", statistics.space.to_string(&problem));
}

#[cfg(test)]
#[test]
pub fn strong_od_problem_2() {
    let f1 = String::from("f1");
    let f2 = String::from("f2");
    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");
    let minit = String::from("minit");
    let init = String::from("init");
    let problem = FONDProblem::new(
        vec![f1.clone(), f2.clone()],
        vec![
            (a.clone(), vec![], vec![
                (vec![f1.clone()], vec![]),
                (vec![f2.clone()], vec![])
            ]),
            (b.clone(), vec![f1.clone()], vec![(vec![f2.clone()], vec![])]),
            (c.clone(), vec![f2.clone()], vec![(vec![f1.clone()], vec![])])
        ],
        vec![
            (
                minit.clone(), init.clone(),
                vec![a.clone(), b.clone(), c.clone()],
                vec![(0,1), (0,2)]
            )
        ],
        vec![init.clone()],
        HashSet::new(),
        init.clone(),
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_strong_od,
    );
    if let AStarResult::Strong(policy) = solution {
        println!("\nPLAN\n");
        println!("{}", policy);
    } else {
        println!("\nNO PLAN\n");
    }
    println!("\nSEARCH SPACE explored:{} total:{}\n", statistics.space.explored_nodes, statistics.space.total_nodes);
    println!("{}", statistics.space.to_string(&problem));
}
