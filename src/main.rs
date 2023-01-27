#![allow(dead_code)]

mod utils;
mod day1;
mod day2;
mod day3;

macro_rules! run {
    ($day:ident) => {{
        use crate::utils::load_file;
        let tmp = load_file(&format!("{}.txt", stringify!($day)));
        println!("Day {}", stringify!($day).strip_prefix("day").unwrap());
        println!("Part 1: {}", $day::part1(&tmp));
        println!("Part 2: {}", $day::part2(&tmp));
        println!();
    }};
}

fn all() {
    run!(day1);
    run!(day2);
    run!(day3);
}

fn main() {
    run!(day3)
}

#[test]
fn test() {
    print!("{:?}", "100 200  300".split(' ').collect::<Vec<&str>>());
}

