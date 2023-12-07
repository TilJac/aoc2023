use std::any::TypeId;

pub fn part_1(input: &str) -> usize{
    let mut lines:Vec<Vec<&str>> = input.lines().map(|x|{x.split_whitespace().collect::<Vec<&str>>()}).collect();
    let num_lines = lines.iter().map(|x|{
        (
            x[0].chars().map(|x|{
                match x{
                    '2' => 2,
                    '3' => 3,
                    '4'=>4,
                    '5'=>5,
                    '6'=>6,
                    '7'=>7,
                    '8'=>8,
                    '9'=>9,
                    'T'=>10,
                    'J'=>11,
                    'Q'=>12,
                    'K'=>13,
                    'A'=>14,
                    _=>{100}
                }
            }).collect::<Vec<usize>>(),
            x[1].parse::<usize>().unwrap()
        )
    });
    let mut types:Vec<Vec<(Vec<usize>, usize)>>= vec![vec![];7];
    let Type_key:Vec<Vec<usize>> = vec![vec![5], vec![1,4], vec![2,3], vec![1,1,3], vec![1,2,2], vec![1,1,1,2], vec![1,1,1,1,1]];
    for z in num_lines{
        let mut nums:Vec<usize> = vec![0];
        let mut line = z.0.clone();
        line.sort();
        let mut last:usize = line[0];
        let mut index = 0;
        for i in line{
            if last == i{
                nums[index] += 1
            }else if last != i{
                index += 1;
                nums.push(1);
                last = i
            }
        }
        nums.sort();
        for (i,x) in Type_key.clone().iter().enumerate(){
            if &nums == x{
                types[i].push(z.clone())
            }
        }

    }
    let mut score = 0;
    let mut multiplier = 1;
    for i in types.iter().rev(){
        if i.len()==1{
            score += i[0].1*multiplier;
            multiplier += 1;
        }else if i.len() !=0{
            let mut typ = i.clone();
            typ.sort_by(|a,b|a.0.cmp(&b.0));
            for i in typ{
                score += i.1*multiplier;
                multiplier += 1
            }
        }
    }
    return score
}

pub fn part_2(input: &str) -> usize{
    let mut lines:Vec<Vec<&str>> = input.lines().map(|x|{x.split_whitespace().collect::<Vec<&str>>()}).collect();
    let num_lines = lines.iter().map(|x|{
        (
            x[0].chars().map(|x|{
                match x{
                    '2' => 2,
                    '3' => 3,
                    '4'=>4,
                    '5'=>5,
                    '6'=>6,
                    '7'=>7,
                    '8'=>8,
                    '9'=>9,
                    'T'=>10,
                    'J'=>1,
                    'Q'=>11,
                    'K'=>12,
                    'A'=>13,
                    _=>{100}
                }
            }).collect::<Vec<usize>>(),
            x[1].parse::<usize>().unwrap()
        )
    });
    let mut types:Vec<Vec<(Vec<usize>, usize)>>= vec![vec![];7];
    let Type_key:Vec<Vec<usize>> = vec![vec![5], vec![1,4], vec![2,3], vec![1,1,3], vec![1,2,2], vec![1,1,1,2], vec![1,1,1,1,1]];
    for z in num_lines{
        let mut nums:Vec<usize> = vec![0];
        let mut line = z.0.clone();
        line.sort();
        let mut last:usize = 100;
        let mut index = 0;
        let mut joker = 0;
        for i in line{
            if i == 1{
                joker += 1;
            }else if last == i || last == 100{
                nums[index] += 1;
                last = i
            }else if last != i{
                index += 1;
                nums.push(1);
                last = i
            }
        }
        nums.sort();
        if let Some(num) = nums.last_mut(){
            *num += joker
        }
        for (i,x) in Type_key.clone().iter().enumerate(){
            if &nums == x{
                types[i].push(z.clone())
            }
        }

    }
    let mut score = 0;
    let mut multiplier = 1;
    for i in types.iter().rev(){
        if i.len()==1{
            score += i[0].1*multiplier;
            multiplier += 1;
        }else if i.len() !=0{
            let mut typ = i.clone();
            typ.sort_by(|a,b|a.0.cmp(&b.0));
            for i in typ{
                score += i.1*multiplier;
                multiplier += 1
            }
        }
    }
    return score
}