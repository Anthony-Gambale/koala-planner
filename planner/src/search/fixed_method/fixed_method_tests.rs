use crate::domain_description::Facts;

use super::*;
use std::{
    borrow::BorrowMut,
    collections::{BTreeSet, HashMap, HashSet},
};

#[cfg(test)]
#[test]
pub fn weak_ld_problem_1() {
    use goal_checks::is_goal_weak_ld;

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
    let (space, plan) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    println!("{}", plan.unwrap());
    println!("\nSEARCH SPACE\n");
    println!("{}", space.to_string(&problem));
}

#[cfg(test)]
#[test]
pub fn weak_ld_problem_2() {
    use goal_checks::is_goal_weak_ld;

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
    let (space, plan) = a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_weak_ld,
    );
    println!("\nPLAN\n");
    println!("{}", plan.unwrap());
    println!("\nSEARCH SPACE\n");
    println!("{}", space.to_string(&problem));
}
