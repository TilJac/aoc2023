pub fn part_1(input: &str) -> isize{
    input.lines().map(|x|{
        x.split(" ").map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>()
    }).map(|x|{get_difference(&x)}).sum()
}

fn get_difference(line: &Vec<isize>) -> isize{
    let last = line.last().unwrap();
    let difference = line.windows(2).map(|x|{x[1]-x[0]}).collect::<Vec<isize>>();
    //println!("{:?}", difference);
    if difference.iter().any(|x|x!=&0){
        let delta = get_difference(&difference);
        return last+delta
    }else{
        return *last
    }
}

pub fn part_2(input: &str) -> isize{
    input.lines().map(|x|{
        x.split(" ").map(|x|x.parse::<isize>().unwrap()).collect::<Vec<isize>>()
    }).map(|x|{get_difference2(&x)}).sum()
}

fn get_difference2(line: &Vec<isize>) -> isize{
    let first = line[0];
    let difference = line.windows(2).map(|x|{x[1]-x[0]}).collect::<Vec<isize>>();
    //println!("{:?}", difference);
    if difference.iter().any(|x|x!=&0){
        let delta = get_difference2(&difference);
        return first-delta
    }else{
        return first
    }
}
