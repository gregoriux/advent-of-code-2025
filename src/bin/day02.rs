use aoc_2025::read_input;
use itertools::Itertools;
use std::env;

fn main() {
    let input_raw = read_input(2);
    let part = env::args().nth(1).unwrap_or("all".to_string());

    let input: Vec<(u64, u64)> = input_raw
        .split(",")
        .filter_map(|segment| {
            segment
                .split("-")
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
        })
        .collect();

    match part.as_str() {
        "1" => solve_part1(&input),
        "2" => solve_part2(&input),
        _ => {
            solve_part1(&input);
            solve_part2(&input);
        }
    }
}

fn solve_part1(input: &[(u64, u64)]) -> () {
    let mut result: u64 = 0;
    for pair in input {
        let a = pair.0;
        let b = pair.1;
        for x in a..=b {
            let digit: u32 = x.ilog10() + 1;
            if digit % 2 == 0 {
                let half_zeros = (10 as u64).pow(digit / 2);
                let first = x / half_zeros;
                let second: u64 = x % half_zeros;
                if first == second {
                    result += x;
                }
            }
        }
    }
    println!("Part 1 Result = {}", result);
}

fn solve_part2(input: &[(u64, u64)]) {
    let mut result: u64 = 0;
    for pair in input {
        let a = pair.0;
        let b = pair.1;
        for x in a..=b {
            let digit: u32 = x.ilog10() + 1;
            let max_segment_size: u32 = digit / 2;
            for segment_size in 1..=max_segment_size {
                if digit % segment_size == 0 {
                    if is_repeating(x, segment_size) {
                        result += x;
                        break;
                    }
                }
            }
        }
    }
    println!("Part 1 Result = {}", result);
}

fn is_repeating(number: u64, segment_size: u32) -> bool {
    let mut p: Option<u64> = None;
    let mut x_temp = number;
    let tens = (10 as u64).pow(segment_size);
    loop {
        let p1 = x_temp % tens;
        if *p.get_or_insert(p1) != p1 {
            break false;
        }
        x_temp = x_temp / tens;
        if x_temp == 0 {
            break true;
        }
    }
}
