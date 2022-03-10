use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use vrp_pragmatic::core::prelude;
use vrp_pragmatic::format::problem::{deserialize_problem, PragmaticProblem, Problem};
use vrp_pragmatic::format::solution::{deserialize_solution, PragmaticSolution, Solution};
use vrp_pragmatic::format::FormatError;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let base_path = args.get(1).expect("please set a valid path to data");
    run(base_path);
}

fn run(base_path: &str) {
    let name = "basic";
    let problem = get_pragmatic_problem(base_path, name);
    let core_problem = Arc::new(problem.clone().read_pragmatic().unwrap_or_else(|errors| {
        panic!(
            "cannot read pragmatic problem: {}",
            FormatError::format_many(errors.as_slice(), "\t\n")
        );
    }));

    let environment = Arc::new(prelude::Environment::default());
    let config = prelude::create_default_config_builder(core_problem.clone(), environment)
        .with_max_generations(Some(100))
        .build()
        .unwrap_or_else(|error| panic!("cannot build defult solver config: {}", error));
    let (solution, cost, _) = prelude::Solver::new(core_problem.clone(), config)
        .solve()
        .unwrap_or_else(|error| panic!("cannot solve problem: {}", error));
    let core_problem =
        Arc::try_unwrap(core_problem).unwrap_or_else(|_| panic!("still has multiple owners"));
    let solution = get_pragmatic_solution(&core_problem, &solution, cost);
    println!("{:?}", solution);
}

fn get_pragmatic_solution(
    problem: &prelude::Problem,
    solution: &prelude::Solution,
    cost: f64,
) -> Solution {
    let mut buffer = String::new();
    let writer = unsafe { BufWriter::new(buffer.as_mut_vec()) };
    (solution, cost)
        .write_pragmatic_json(problem, writer)
        .expect("cannot write solution");
    deserialize_solution(BufReader::new(buffer.as_bytes())).expect("cannot deserialize solution")
}

fn get_pragmatic_problem(base_path: &str, name: &str) -> Problem {
    deserialize_problem(open_file(
        format!["{}/{}.problem.json", base_path, name].as_str(),
    ))
    .unwrap()
}

fn open_file(path: &str) -> BufReader<File> {
    println!("Reading '{}'", path);
    BufReader::new(File::open(path).unwrap_or_else(|err| panic!("cannot open {}: {}", path, err)))
}
