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
    //println!("{:?}", keys);
    let mut best_location = usize::MAX;
    for i in seeds{
        let mut id = i;
        for z in 0..=6{
            //println!("-------------- {z}");
            for x in &keys[z]{
                //println!("{:?}", x);
                if {id > x[1]|| id == x[1] }&& id< x[1]+x[2]{
                    //println!("{z}, {id}, {:?}", x);
                    let difference = id - x[1];
                    id = x[0] + difference;
                    //println!("{id}, {difference}, {:?}", x);
                    break;
                }
            }
            //println!("end of cycle: {id}");
        }
        //println!("id : {id}");
        if id < best_location{
            best_location = id;
        }
    }
    return best_location
}

pub fn part_2(input: &str) -> usize{
    let mut parts = input.split("\n\n").map(|x|{
        x.split(":").collect::<Vec<&str>>()[1]
    });
    let seeds1:Vec<usize> = parts.next().unwrap().split_ascii_whitespace().map(|x|{
        x.parse::<usize>().unwrap()
    }).collect();
    let keys: Vec<Vec<Vec<usize>>> = parts.map(|x|{
        x.trim().split("\n").map(|x|{
            x.split(" ").map(|x|{
                x.parse::<usize>().unwrap()
            }).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>()
    }).collect();
    //println!("{:?}", keys);
    let mut best_location = usize::MAX;
    for i in 0..seeds1.len()/{2}{
        for i in seeds1[i*2]..seeds1[i*2]+seeds1[i*2+1]{
            let mut id = i;
            for z in 0..=6{
                //println!("-------------- {z}");
                for x in &keys[z]{
                    //println!("{:?}", x);
                    if {id > x[1]|| id == x[1] }&& id< x[1]+x[2]{
                        //println!("{z}, {id}, {:?}", x);
                        let difference = id - x[1];
                        id = x[0] + difference;
                        //println!("{id}, {difference}, {:?}", x);
                        break;
                    }
                }
                //println!("end of cycle: {id}");
            }
            //println!("id : {id}");
            if id < best_location{
                best_location = id;
            }
        }
    }
    return best_location
}