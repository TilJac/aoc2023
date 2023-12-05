use day_5::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    println!("--- Day 5: If You Give A Seed A Fertilizer ---");
    println!("first puzzle: {}", part_1(input));
    println!("second puzzle: {}", part_2(input));
}
