use day_4::{part_1, part_2};

fn main() {
    let input = include_str!("../input.txt");
    println!("--- Day 4: Scratchcards ---");
    println!("first puzzle: {}", part_1(input));
    println!("second puzzle: {}", part_2(input));
}
