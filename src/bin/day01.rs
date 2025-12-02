use aoc_2025::read_input;
use std::env;

fn main() {
    let input = read_input(1);
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
    let mut total: i64 = 50;
    let mut result: u32 = 0;

    for rotation in input.split_ascii_whitespace() {
        let val = rotation[1..].parse::<i64>().unwrap();

        match rotation.as_bytes()[0] {
            b'L' => total -= val,
            _ => total += val,
        }

        if total.rem_euclid(100) == 0 {
            result += 1;
        }
    }
    println!("Part 1 Result = {}", result);
}

fn solve_part2(input: &str) {
    let mut total: i64 = 50;
    let mut result: i64 = 0;

    for rotation in input.split_ascii_whitespace() {
        let val = rotation[1..].parse::<i64>().unwrap();
        let previous = total;
        match rotation.as_bytes()[0] {
            b'L' => total -= val,
            _ => total += val,
        }
        let crossed = if total > previous {
            total.div_euclid(100) - previous.div_euclid(100)
        } else {
            (previous - 1).div_euclid(100) - (total - 1).div_euclid(100)
        };

        result += crossed.abs();
    }
    println!("Part 2 Result = {}", result);
}
