use crate::domain_description::Facts;
use crate::domain_description::FONDProblem;
use search_node::get_successors_systematic;
use goal_checks::*;
use super::*;
use std::{
    borrow::BorrowMut,
    collections::{BTreeSet, HashMap, HashSet}, vec,
};
use astar::{a_star_search, AStarResult};

#[cfg(test)]
#[test]
pub fn weak_ld_problem_1() {
    let problem = FONDProblem::new(
        vec![],
        vec![
            (String::from("prim_a"), vec![], vec![]),
            (String::from("prim_b"), vec![], vec![]),
            (String::from("prim_e"), vec![], vec![]),
            (String::from("prim_x"), vec![], vec![]),
        ],
        vec![
            (
                String::from("m0"),
                String::from("comp_init"),
                vec![String::from("prim_a"), String::from("comp_c"), String::from("prim_x")],
                vec![(0,1), (1,2)],
            ),
            (
                String::from("m1"),
                String::from("comp_c"),
                vec![String::from("prim_b"), String::from("comp_d")],
                vec![(0,1)]
            ),
            (
                String::from("m2"),
                String::from("comp_d"),
                vec![String::from("prim_e")],
                vec![]
            )
        ],
        vec![String::from("comp_init"), String::from("comp_c"), String::from("comp_d")],
        HashSet::new(),
        String::from("comp_init"),
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    if let AStarResult::Linear(lin) = solution {
        println!("{}", lin.to_string(&problem));
    } else {
        println!("NO SOLUTION");
    }
    println!("\nSEARCH SPACE explored:{} total:{}\n", statistics.space.explored_nodes, statistics.space.total_nodes);
    println!("{}", statistics.space.to_string(&problem));
}

#[cfg(test)]
#[test]
pub fn weak_ld_problem_2() {
    let f1: String = String::from("f1");
    let f2: String = String::from("f2");
    let f3: String = String::from("f3");
    let problem = FONDProblem::new(
        vec![f1.clone(), f2.clone(), f3.clone()],
        vec![
            (
                String::from("a"),
                vec![],
                vec![(vec![], vec![f2.clone()]), (vec![], vec![])],
            ),
            (
                String::from("b"),
                vec![],
                vec![(vec![f3.clone()], vec![f2.clone()])],
            ),
        ],
        vec![
            (
                String::from("m0"),
                String::from("init"),
                vec![String::from("a"), String::from("b"), String::from("c")],
                vec![(0, 2), (1, 2)],
            ),
            (
                String::from("m1"),
                String::from("c"),
                vec![String::from("a"), String::from("c")],
                vec![(0, 1)],
            ),
            (
                String::from("m2"),
                String::from("c"),
                vec![String::from("a")],
                vec![],
            ),
        ],
        vec![String::from("c"), String::from("init")],
        HashSet::from([f1.clone(), f2.clone()]),
        String::from("init"),
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    if let AStarResult::Linear(lin) = solution {
        println!("{}", lin.to_string(&problem));
    } else {
        println!("NO SOLUTION");
    }
    println!("\nSEARCH SPACE explored:{} total:{}\n", statistics.space.explored_nodes, statistics.space.total_nodes);
    println!("{}", statistics.space.to_string(&problem));
}

#[cfg(test)]
#[test]
pub fn test_deordering() {
    // actions
    let a1 = String::from("A");
    let a3 = String::from("B");
    let a5 = String::from("C");
    let a6 = String::from("B"); // same action as a3
    // compound names
    let init = String::from("init");
    let c2 = String::from("COMPOUND1");
    let c4 = String::from("COMPOUND2");
    // method names
    let minit = String::from("minit");
    let m1 = String::from("m1");
    let m2 = String::from("m2");
    // fond problem
    let problem = FONDProblem::new(
        vec![], // no facts needed
        vec![
            (a1.clone(), vec![], vec![]),
            (a3.clone(), vec![], vec![]),
            (a5.clone(), vec![], vec![]),
            (a6.clone(), vec![], vec![])
        ],
        vec![
            (
                minit.clone(), init.clone(),
                vec![a1.clone(), c2.clone()],
                vec![(0,1)]
            ),
            (
                m1.clone(), c2.clone(),
                vec![a3.clone(), c4.clone(), a5.clone()],
                vec![(0,1),(1,2)]
            ),
            (
                m2.clone(), c4.clone(),
                vec![a6.clone()],
                vec![]
            )
        ],
        vec![c2.clone(), c4.clone(), init.clone()],
        HashSet::new(),
        init.clone()
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    let goal = statistics.goal_node.unwrap();
    if let AStarResult::Linear(lin) = solution {
        println!("{}", lin.to_string(&problem));
    } else {
        println!("NO SOLUTION");
    }
    println!("\nDE-ORDERED TASK NETWORK\n");
    let de = deorder(goal);
    for (k, v) in de.get_orderings() {
        let kstring: String = de.get_task(k).borrow().get_name();
        let vstring: String = de.get_task(v).borrow().get_name();
        let kprime = format!("{}:{}", kstring, k);
        let vprime = format!("{}:{}", vstring, v);
        println!("{} < {}", kprime, vprime);
    }
}

#[cfg(test)]
#[test]
pub fn test_deordering2() {
    // primitive names
    let a = String::from("a");
    let b = String::from("b");
    let d = String::from("d");
    let e = String::from("e");
    // compound names
    let init = String::from("init");
    let c1 = String::from("comp_1");
    let c2 = String::from("comp_2");
    // method names
    let minit = String::from("minit");
    let m1 = String::from("m1");
    let m2 = String::from("m2");
    // fond problem
    let problem = FONDProblem::new(
        vec![], // no facts needed
        vec![
            (a.clone(), vec![], vec![]),
            (b.clone(), vec![], vec![]),
            (d.clone(), vec![], vec![]),
            (e.clone(), vec![], vec![])
        ],
        vec![
            (
                minit, init.clone(),
                vec![a.clone(), c1.clone(), c2.clone(), b.clone()],
                vec![(0,1), (0,2), (1,3)]
            ),
            (
                m1, c1.clone(),
                vec![a.clone(), d.clone()],
                vec![]
            ),
            (
                m2, c2.clone(),
                vec![e.clone()],
                vec![]
            )
        ],
        vec![c1.clone(), c2.clone(), init.clone()],
        HashSet::new(),
        init.clone()
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    let goal = statistics.goal_node.unwrap();
    if let AStarResult::Linear(lin) = solution {
        println!("{}", lin.to_string(&problem));
    } else {
        println!("NO SOLUTION");
    }
    println!("\nDE-ORDERED TASK NETWORK\n");
    let de = deorder(goal);
    for (k, v) in de.get_orderings() {
        let kstring: String = de.get_task(k).borrow().get_name();
        let vstring: String = de.get_task(v).borrow().get_name();
        let kprime = format!("{}:{}", kstring, k);
        let vprime = format!("{}:{}", vstring, v);
        println!("{} < {}", kprime, vprime);
    }
}

#[cfg(test)]
#[test]
fn test_deordering3() {
    // primitive names
    let a = String::from("a");
    let b = String::from("b");
    let noop = String::from("noop");
    // compound names
    let t = String::from("t");
    let init = String::from("init");
    // fact names
    let f1 = String::from("f1");
    let f2 = String::from("f2");
    let f3 = String::from("f3");
    // method names
    let m1 = String::from("m1");
    let m2 = String::from("m2");
    let minit = String::from("minit");
    // fond problem
    let problem = FONDProblem::new(
        vec![f1.clone(), f2.clone(), f3.clone()],
        vec![
            (
                a.clone(), vec![], vec![
                    (vec![f1.clone()], vec![]),
                    (vec![f2.clone()], vec![]),
                    (vec![f3.clone()], vec![])
                ]
            ),
            (
                b.clone(), vec![f1.clone(), f2.clone(), f3.clone()], vec![(vec![], vec![])]
            ),
            (
                noop.clone(), vec![], vec![(vec![], vec![])]
            )
        ],
        vec![
            (
                minit.clone(), init.clone(),
                vec![t.clone(), b.clone()],
                vec![(0,1)]
            ),
            (
                m1.clone(), t.clone(),
                vec![a.clone(), t.clone()],
                vec![(0,1)]
            ),
            (
                m2.clone(), t.clone(),
                vec![noop.clone()], vec![]
            )
        ],
        vec![init.clone(), t.clone()],
        HashSet::new(),
        init.clone()
    );
    let (solution, statistics) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nSPACE\n");
    println!("{}", statistics.space.to_string(&problem));
    println!("\nPLAN\n");
    let goal = statistics.goal_node.unwrap();
    if let AStarResult::Linear(lin) = solution {
        println!("{}", lin.to_string(&problem));
    } else {
        println!("NO SOLUTION");
    }
    println!("\nDE-ORDERED TASK NETWORK\n");
    let de = deorder(goal);
    for (k, v) in de.get_orderings() {
        let kstring: String = de.get_task(k).borrow().get_name();
        let vstring: String = de.get_task(v).borrow().get_name();
        let kprime = format!("{}:{}", kstring, k);
        let vprime = format!("{}:{}", vstring, v);
        println!("{} < {}", kprime, vprime);
    }
}
