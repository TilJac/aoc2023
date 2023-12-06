pub fn part_1(input: &str) -> usize{
    let x = input.lines().map(|x|{x.split(":").collect::<Vec<&str>>()[1]
        .split(' ').filter(|x|{!x.is_empty()})
        .map(|x|{x.parse::<usize>().unwrap()})
        .collect::<Vec<usize>>()}).collect::<Vec<Vec<usize>>>();
    let number = x[0].len();
    let mut result:usize = 1;
    for i in 0..number{
        let mut number_of_possibilities: usize = 0;
        for z in 0..=x[0][i]{
            if z * (x[0][i]-z) > x[1][i]{
                number_of_possibilities +=1;
            }
        }
        result *= number_of_possibilities;
    }

    return result
}

pub fn part_2(input: &str) -> usize{
    let x = input.lines().map(|x|{x.split(":").collect::<Vec<&str>>()[1].replace(" ", "").parse::<usize>().unwrap()}).collect::<Vec<usize>>();
    let mut number_of_possibilities: usize = 0;
    for z in 0..=x[0]{
        if z * (x[0]-z) > x[1]{
            number_of_possibilities +=1;
        }
    }

    return number_of_possibilities
}