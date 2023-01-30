mod solutions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_no = &args[1];
    solve(solution_no);
}

fn solve(no: &str) {
    match no {
        "02001" => solutions::s02001::solve(),
        _ => println!("solution {} is not found.", no),
    }
}
