use day_8::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    println!("--- Day 8: Haunted Wasteland ---");
    println!("first puzzle: {}", part_1(input));
    println!("second puzzle: {}", part_2(input));
}
