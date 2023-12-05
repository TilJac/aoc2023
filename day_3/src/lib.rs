pub fn part_1(input: &str) -> usize{
    let symbols:Vec<Vec<usize>> = input.lines().map(|x|{
        let mut coordinate = vec![];
        for (i, x) in x.chars().enumerate(){
            if x=='.' || x.is_ascii_digit(){
                continue;
            }else{
                println!("{x}");
                coordinate.push(i);
            }
        }
        coordinate
    }).collect();
    let mut sum = 0;
    
    let mut line = input.lines().map(|x|{x.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    let mut lines = vec![line[0].clone()];
    lines.append(&mut line);
    lines.push(lines[0].clone());
    for (i, x) in input.lines().enumerate(){
        let mut temporary = String::new();
        let mut min: usize = 0;
        let mut max: usize = 0;
        let slice = if i == 0{&symbols[0..=1]}else if i == symbols.len()-1{&symbols[i-1..=i]}else{&symbols[i-1..=i+1]};
        for (z, x) in x.chars().enumerate() {
            if x.is_ascii_digit(){
                temporary += &x.to_string();
                max = z;
                if min ==0 {
                    min = z;
                }
            }else if temporary != String::new(){
                if check_coordinate(slice, min , max){
                    sum += usize::from_str_radix(&temporary, 10).unwrap();
                }else{
                    println!("{:?}{:?}{:?}", &lines[i][min-1..=max+1], &lines[i+1][min-1..=max+1], &lines[i+2][min-1..=max+1])
                }
                temporary = String::new(); 
                min=0;
                max = 0;
            }
        }
    }
    return sum
}

fn check_coordinate(coordinates: &[Vec<usize>], min:usize, max: usize) -> bool{
    for i in coordinates{
        for &x in i{
            if x > max + 1{
                break;
            }else if x < min - 1{
                continue;
            }else {
                return true
            }
        }
    }
    return false;
}

pub fn part_2(input: &str) -> usize{
    return 2
}