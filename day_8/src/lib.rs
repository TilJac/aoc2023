use std::collections::HashMap;
pub fn part_1(input: &str) -> usize{
    let mut list:HashMap<&str, [String;2]> = HashMap::new();
    let input: Vec<&str> = input.split("\n\n").collect();
    let instructions:Vec<usize> = input[0].chars().map(|x|{
        match x{
            'R' => 1,
            'L'=> 0,
            _ => 100
        }
    }).collect();
    input[1].lines().for_each(|x|{
        let line: Vec<&str> = x.split(" = ").collect();
        let vars:Vec<String> = line[1].split(", ").map(|x|{let x = x.replace('(', "").replace(')', "");x}).collect::<Vec<String>>();
        list.insert(line[0], [vars[0].clone(), vars[1].clone()]);
    });
    let mut current = "AAA";
    let mut iteration = 0;
    let len = instructions.len(); 
    while current != "ZZZ"{
        current=list[current][instructions[iteration%len]].as_str();
        iteration += 1;
    }
    return iteration 
}

pub fn part_2(input: &str) -> u128{
    let mut list:HashMap<&str, [String;2]> = HashMap::new();
    let input: Vec<&str> = input.split("\n\n").collect();
    let instructions:Vec<usize> = input[0].chars().map(|x|{
        match x{
            'R' => 1,
            'L'=> 0,
            _ => 100
        }
    }).collect();
    let mut startingpoints = vec![];
    input[1].lines().for_each(|x|{
        let line: Vec<&str> = x.split(" = ").collect();
        let vars:Vec<String> = line[1].split(", ").map(|x|{let x = x.replace('(', "").replace(')', "");x}).collect::<Vec<String>>();
        if line[0].chars().last() == Some('A'){
            startingpoints.push(line[0])
        }
        list.insert(line[0], [vars[0].clone(), vars[1].clone()]);
    });
    let mut status = false;
    let mut iteration = 0;
    let len = instructions.len(); 
    let mut startingpoints = startingpoints;
    println!("{:?}", startingpoints);
    let mut solutions: Vec<(u128, u128)> = startingpoints.iter().map(|&current|{
        let mut current = current;
        let mut iteration = 0;
        let len = instructions.len(); 
        while !current.ends_with('Z'){
            current=list[current][instructions[iteration%len]].as_str();
            iteration += 1;
        }
        (iteration as u128, iteration as u128)
    }).collect();
    println!("{:?}", solutions);
    let solutions2 = solutions.clone();
    while solutions.iter().map(|x|{x.0}).collect::<Vec<u128>>() != vec![solutions[0].0;solutions.len()]{
        println!("{:?}", solutions);
        solutions.sort_by(|x,b|{x.0.cmp(&b.0)});
        solutions[0].0 += solutions[0].1;
    }
    return solutions[0].0
}


