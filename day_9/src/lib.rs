pub fn part_1(input: &str) -> isize{
    input.lines().map(|x|{
        x.split(" ").map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>()
    }).map(|x|{get_difference(&x)}).sum()
}

fn get_difference(line: &Vec<isize>) -> isize{
    let mut last = line[0];
    let mut difference = vec![]; // can be replaced with a single value + a bool
    for &i in line.iter().skip(1){
        difference.push(i-last);
        last = i
    }
    println!("{:?}", difference);
    if difference != vec![0 as isize; difference.len()]{
        let delta = get_difference(&difference);
        return last+delta
    }else{
        return last
    }
}

pub fn part_2(input: &str) -> isize{
    input.lines().map(|x|{
        x.split(" ").map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>()
    }).map(|x|{get_difference2(&x)}).sum()
}

fn get_difference2(line: &Vec<isize>) -> isize{
    let first = line[0];
    let mut last = line[0];
    let mut difference = vec![]; // can be replaced with a single value + a bool
    for &i in line.iter().skip(1){
        difference.push(i-last);
        last = i
    }
    println!("{:?}", difference);
    if difference != vec![0 as isize; difference.len()]{
        let delta = get_difference2(&difference);
        return first-delta
    }else{
        return first
    }
}
