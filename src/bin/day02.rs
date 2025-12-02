use aoc_2025::read_input;
use std::env;

fn main() {
    let input = read_input(2);
    let part = env::args().nth(1).unwrap_or("all".to_string());

    match part.as_str() {
        "1" => solve_part1(&input),
        "2" => solve_part2(&input),
        _ => {
            solve_part1(&input);
            solve_part2(&input);
        }
    }
}

fn solve_part1(input: &str) {
    
}

fn solve_part2(input: &str) {}
