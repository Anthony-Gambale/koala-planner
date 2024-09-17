#![allow(unused)]
use std::{collections::{HashSet, HashMap}, env};

extern crate bit_vec;

mod domain_description;
mod graph_lib;
mod heuristics;
mod relaxation;
mod search;
mod task_network;

use crate::search::fixed_method::heuristic_factory;
use crate::search::{HeuristicType, SearchResult};
use domain_description::{read_json_domain, FONDProblem};
use heuristics::{h_add, h_ff, h_max};
use relaxation::RelaxedComposition;
use search::{
    astar::AStarResult,
    goal_checks::{is_goal_strong_od, is_goal_weak_ld},
    search_node::{get_successors_systematic, SearchNode},
};
use task_network::HTN;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("The path to the problem file is not given.");
        return;
    }
    let problem = read_json_domain(&args[1]);

    let heuristic = match args.get(3) {
        Some(flag) => match flag.as_str() {
            "--add" => {
                println!("Using Add heuristic");
                heuristic_factory::create_function_with_heuristic(h_add)
            },
            "--max" => {
                println!("Using Max heuristic");
                heuristic_factory::create_function_with_heuristic(h_max)
            },
            "--ff" => {
                println!("Using FF heuristic");
                heuristic_factory::create_function_with_heuristic(h_ff)
            },
            _ => panic!("Did not recognise flag {}", flag),
        },
        None => {
            println!("Using constant zero heuristic");
            heuristic_factory::zero_heuristic()
        }
    };

    match args.get(2) {
        Some(flag) => match flag.as_str() {
            "--fixed" => fixed_method(&problem, heuristic),
            "--flexible" => method_based(&problem),
            _ => panic!("Did not recognise flag {}", flag),
        },
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
            // if (stats.search_nodes < 30) {
            //     println!("***************************");
            //     println!("{}", x);
            // }
        }
        SearchResult::NoSolution => {
            println!("Problem has no solution");
        }
    }
}

fn fixed_method(problem: &FONDProblem, heuristic: heuristic_factory::HeuristicFn) {
    let (solution, stats) = search::fixed_method::astar::a_star_search(
        &problem,
        heuristic,
        get_successors_systematic,
        || 1.0,
        is_goal_strong_od,
    );
    println!("{}", stats);
    // println!(
    //     "Number of maybe-isomorphic buckets: {}",
    //     stats.space.maybe_isomorphic_buckets.len()
    // );
    if let AStarResult::Strong(policy) = solution {
        println!("Solution was found");
        // if (stats.space.total_nodes < 30) {
        //     println!("***************************");
        //     println!("{}", policy);
        // }
    } else {
        println!("Problem has no solution");
    }
    // if (stats.space.total_nodes < 30) {
    //     println!("{}", stats.space.to_string(problem));
    // }
}
