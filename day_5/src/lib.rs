pub fn part_1(input: &str) -> usize{
    let mut parts = input.split("\n\n").map(|x|{
        x.split(":").collect::<Vec<&str>>()[1]
    });
    let seeds:Vec<usize> = parts.next().unwrap().split_ascii_whitespace().map(|x|{
        x.parse::<usize>().unwrap()
    }).collect();
    let keys: Vec<Vec<Vec<usize>>> = parts.map(|x|{
        x.trim().split("\n").map(|x|{
            x.split(" ").map(|x|{
                x.parse::<usize>().unwrap()
            }).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>()
    }).collect();
    println!("{:?}", keys);
    return 1
}

pub fn part_2(input: &str) -> usize{
    return 2
}