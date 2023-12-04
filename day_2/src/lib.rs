pub fn part_1(input: &str) -> usize{
    input.lines().map(|x|{
        let line: Vec<&str> = x.split(':').collect();
        let sections:Vec<Vec<usize>> = line[1].split(';').map(|x|{
            let mut list:Vec<usize> = vec![0,0,0];
            for x in x.split(','){
                let argument:Vec<&str> = x.trim().split(' ').collect();
                if argument[1] == "red"{
                    list[0]=usize::from_str_radix(argument[0], 10).unwrap()
                }else if argument[1] == "green"{
                    list[1]=usize::from_str_radix(argument[0], 10).unwrap()
                }else if argument[1] == "blue"{
                    list[2]=usize::from_str_radix(argument[0], 10).unwrap()
                }
            }
            list
        }).collect();
        let game: Vec<String> = line[0].chars().filter(|x|x.is_ascii_digit()).map(|x|x.to_string()).collect();
        let mut id = usize::from_str_radix(&game.join(""), 10).unwrap();
        for i in sections{
            if i[0] > 12||i[1]>13||i[2]>14{
                id = 0
            }
        }
        id

    }).sum()
}

pub fn part_2(input: &str) -> usize{
    input.lines().map(|x|{
        let line: Vec<&str> = x.split(':').collect();
        let sections:Vec<Vec<usize>> = line[1].split(';').map(|x|{
            let mut list:Vec<usize> = vec![0,0,0];
            for x in x.split(','){
                let argument:Vec<&str> = x.trim().split(' ').collect();
                if argument[1] == "red"{
                    list[0]=usize::from_str_radix(argument[0], 10).unwrap()
                }else if argument[1] == "green"{
                    list[1]=usize::from_str_radix(argument[0], 10).unwrap()
                }else if argument[1] == "blue"{
                    list[2]=usize::from_str_radix(argument[0], 10).unwrap()
                }
            }
            list
        }).collect();
        let mut iter = sections.iter();
        let mut lowest = iter.next().unwrap().to_owned();
        for i in iter{
            println!("{:?}",lowest);
            if i[0]>lowest[0]{
                lowest[0]=i[0]
            }
            if i[1]>lowest[1]{
                lowest[1]=i[1]
            }
            if i[2]>lowest[2]{
                lowest[2]=i[2]
            }
        }
        lowest[0]*lowest[1]*lowest[2]

    }).sum()
}