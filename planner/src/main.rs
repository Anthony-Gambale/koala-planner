#![allow(unused)]
use std::env;

extern crate bit_vec;

mod domain_description;
mod graph_lib;
mod task_network;
mod search;
mod relaxation;
mod heuristics;

use domain_description::{read_json_domain, FONDProblem};
use search::{astar::AStarResult, goal_checks::{is_goal_strong_od, is_goal_weak_ld}, search_node::{get_successors_systematic, SearchNode}};
use crate::search::{SearchResult, HeuristicType};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("The path to the problem file is not given.");
        return;
    }
    let problem = read_json_domain(&args[1]);
    match args.get(2) {
        Some(flag) => match flag.as_str() {
            "-f" => fixed_method(&problem),
            _ => panic!("Did not recognise flag {}", flag),
        }
        None => method_based(&problem),
    }
}

fn method_based(problem: &FONDProblem) {
    let (solution, stats) = search::AOStarSearch::run(problem, HeuristicType::HAdd);
    print!("{}", stats);
    match solution {
        SearchResult::Success(x) => {
            println!("makespan: {}", x.makespan);
            println!("policy enteries: {}", x.transitions.len());
            //println!("***************************");
            //println!("{}", x);
        },
        SearchResult::NoSolution => {
            println!("Problem has no solution");
        }
    }
}

fn fixed_method(problem: &FONDProblem) {
    let (solution, stats) = search::fixed_method::astar::a_star_search(
        &problem,
        |x, y, z| 0.0,
        get_successors_systematic,
        || 1.0,
        is_goal_strong_od,
    );
    print!("{}", stats);
    if let AStarResult::Strong(policy) = solution {
        println!("{}", policy);
    } else {
        println!("Problem has no solution");
    }
}
