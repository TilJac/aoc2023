#![feature(isqrt)]
pub fn part_1(input: &str) -> isize{
    let lines = input.lines().map(|x|x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut columns = vec![];
    'outer: for i in 0..lines[0].len(){
        for z in &lines{
            if z[i] == '#'{
                continue 'outer
            }
        }
        columns.push(i)
    }
    let mut stars = vec![];
    let mut y_coord = 0;
    for x in lines.iter(){
        let mut x_coord = 0;
        let mut star_in_column = false;
        for (j, &y) in x.iter().enumerate(){
            if y == '#'{
                stars.push((y_coord, x_coord));
                star_in_column = true
            }
            x_coord += 1;
            if columns.contains(&j){
                x_coord += 999999;
            }
        }
        y_coord += 1;
        if !star_in_column{
            y_coord += 999999;
        }
    }
    let mut total = 0;
    let mut number = 0;
    for (i, &x) in stars.iter().enumerate(){
        for &z in &stars[i+1..]{
            total += isize::abs(x.0-z.0) + isize::abs(x.1-z.1);
            number += 1;
        }
    }
    return total
}

pub fn part_2(input: &str) -> isize{
    let lines = input.lines().map(|x|x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut columns = vec![];
    'outer: for i in 0..lines[0].len(){
        for z in &lines{
            if z[i] == '#'{
                continue 'outer
            }
        }
        columns.push(i)
    }
    let mut stars = vec![];
    let mut y_coord = 0;
    for x in lines.iter(){
        let mut x_coord = 0;
        let mut star_in_column = false;
        for (j, &y) in x.iter().enumerate(){
            if y == '#'{
                stars.push((y_coord, x_coord));
                star_in_column = true
            }
            x_coord += 1;
            if columns.contains(&j){
                x_coord += 1;
            }
        }
        y_coord += 1;
        if !star_in_column{
            y_coord += 1;
        }
    }
    let mut total = 0;
    let mut number = 0;
    for (i, &x) in stars.iter().enumerate(){
        for &z in &stars[i+1..]{
            total += isize::abs(x.0-z.0) + isize::abs(x.1-z.1);
            number += 1;
        }
    }
    return total
}