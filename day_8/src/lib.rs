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
    println!("{:?}", input);
    input[1].lines().for_each(|x|{
        let line: Vec<&str> = x.split(" = ").collect();
        println!("line: {:?}", line);
        let vars:Vec<String> = line[1].split(", ").map(|x|{let x = x.replace('(', "").replace(')', "");x}).collect::<Vec<String>>();
        println!("vars:{:?}", vars);
        list.insert(line[0], [vars[0].clone(), vars[1].clone()]);
    });
    let mut current = "AAA";
    let mut iteration = 0;
    let len = instructions.len(); // have to check if this is right
    println!("{:?}, {:?}", list, instructions);
    while current != "ZZZ"{
        println!("{}", iteration%len);
        println!("{current}");
        match list.get(current){
            
            Some(x) => {current=x[instructions[iteration%len]].as_str()},
            None => print!("error")
        }
        iteration += 1;
    }
    return iteration 
}

pub fn part_2(input: &str) -> usize{
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
    while status{
        status = true;
        startingpoints = startingpoints.iter().map(|&i|{let value = list[i][instructions[iteration%len]].as_str(); if !value.ends_with('Z'){status = false} value}).collect();
        iteration += 1;
    }
    return iteration 
}


