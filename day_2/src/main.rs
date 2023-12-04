use day_1::{part_1, part_2};

fn main() {
    let input = include_str!("../test_1.txt");
    println!("--- Day 2: Cube Conundrum ---");
    println!("first puzzle: {}", part_1(input));
    println!("second puzzle: {}", part_2(input));
}
