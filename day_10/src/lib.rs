#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Tile {
    Ver,
    Hor,
    N_E,
    N_W,
    S_W,
    S_E,
    Emp,
    Sta,
    Loo, 
    Out,  
    Inn
}

impl Tile{
    fn get_next(&self, last: (isize, isize)) -> (isize, isize){
        //println!("{:?}", last);
        //println!("self: {:?}", self);
        let potential:((isize, isize), (isize, isize)) = match self{
            Tile::Ver => ((1,0),(-1,0)),
            Tile::Hor => ((0,1),(0,-1)),
            Tile::N_E => ((-1,0),(0,1)),
            Tile::N_W => ((-1,0),(0,-1)),
            Tile::S_W => ((1,0),(0,-1)),
            Tile::S_E => ((1,0),(0,1)),
            Tile::Sta => panic!("start"),
            _ => panic!("shouldn't be here")
        };
        if (-1*last.0,-1*last.1) == potential.0{
            return potential.1
        }else{
            return potential.0
        }
    }
}

fn char_to_tile(char: char) -> Tile{
    match char{
        '|' => Tile::Ver,
        '-' => Tile::Hor,
        'L' => Tile::N_E,
        'J' => Tile::N_W,
        '7' => Tile::S_W,
        'F' => Tile::S_E,
        '.' => Tile::Emp,
        'S' => Tile::Sta,
        _ => panic!("shouldn't happen")
    }
}

pub fn part_1(input: &str) -> usize{
    let tiles = input.lines().map(|x|{
        x.chars().map(|x|char_to_tile(x)).collect::<Vec<Tile>>()
    }).collect::<Vec<Vec<Tile>>>();
    let mut start = (0,0);
    'outer: for (i,x) in tiles.iter().enumerate(){
        for (j, x) in x.iter().enumerate(){
            if *x == Tile::Sta{
                start = (i, j);
                break 'outer;
            }
        }
    }
    //println!("{:?}", tiles);
    let mut startingpath_1 = (0,0);
    let mut startingpath_2 = (0,0);
    if start.0 >0 && {tiles[start.0-1][start.1] == Tile::Ver||tiles[start.0-1][start.1] ==Tile::S_E||tiles[start.0-1][start.1] ==Tile::S_W}{
        startingpath_1 = (start.0 -1, start.1)
    }
    if start.0 < tiles.len() && {tiles[start.0+1][start.1] == Tile::Ver||tiles[start.0+1][start.1] ==Tile::N_E||tiles[start.0+1][start.1] ==Tile::N_W}{
        if startingpath_1 ==(0,0){
            startingpath_1 = (start.0 + 1, start.1)
        }else{
            startingpath_2 = (start.0 + 1, start.1)

        }
    }
    if start.1 > 0 && {tiles[start.0][start.1-1] == Tile::Hor||tiles[start.0][start.1-1] ==Tile::S_E||tiles[start.0][start.1-1] ==Tile::N_E}{
        if startingpath_1 ==(0,0){
            startingpath_1 = (start.0, start.1-1)
        }else{
            startingpath_2 = (start.0, start.1-1)
        }
    }
    if start.1 < tiles[0].len() && {tiles[start.0][start.1+1] == Tile::Hor||tiles[start.0][start.1+1] ==Tile::S_W||tiles[start.0][start.1+1] ==Tile::N_W}{
        startingpath_2 = (start.0, start.1+1)
    }
    let startingpath_1 = (startingpath_1.0 as isize, startingpath_1.1 as isize);
    let startingpath_2 = (startingpath_2.0 as isize, startingpath_2.1 as isize);
    //println!("{:?}", start);
    //println!("{:?}, {:?}", startingpath_1, startingpath_2);
    //println!("{:?}, {:?}", tiles[startingpath_1.0 as usize][startingpath_1.1 as usize].get_next((-1*start.0 as isize+startingpath_1.0, -1*start.1 as isize+ startingpath_1.1)), 
    tiles[startingpath_2.0 as usize][startingpath_2.1 as usize].get_next((-1*start.0 as isize+startingpath_2.0, -1*start.1 as isize+ startingpath_2.1));
    let mut last = (startingpath_1.0 - start.0 as isize, startingpath_1.1 - start.1 as isize);
    let mut current =({start.0 as isize + last.0} as usize, {start.1 as isize + last.1} as usize);
    //println!("current: {:?}, {:?}", current, last); 
    let mut count = 1;
    loop{
        last = tiles[current.0][current.1].get_next(last);
        current = ({current.0 as isize + last.0} as usize, {current.1 as isize + last.1} as usize);
        //println!("here: {:?}, {:?}", last, current);
        count+=1;
        if tiles[current.0][current.1] == Tile::Sta{
            break;
        }
    }
    return count/2
}

pub fn part_2(input: &str) -> isize{
    let tiles = input.lines().map(|x|{
        x.chars().map(|x|char_to_tile(x)).collect::<Vec<Tile>>()
    }).collect::<Vec<Vec<Tile>>>();
    let mut start = (0,0);
    'outer: for (i,x) in tiles.iter().enumerate(){
        for (j, x) in x.iter().enumerate(){
            if *x == Tile::Sta{
                start = (i, j);
                break 'outer;
            }
        }
    }
    //println!("{:?}", tiles);
    let mut startingpath_1 = (0,0);
    let mut startingpath_2 = (0,0);
    if start.0 >0 && {tiles[start.0-1][start.1] == Tile::Ver||tiles[start.0-1][start.1] ==Tile::S_E||tiles[start.0-1][start.1] ==Tile::S_W}{
        startingpath_1 = (start.0 -1, start.1)
    }
    if start.0 < tiles.len() && {tiles[start.0+1][start.1] == Tile::Ver||tiles[start.0+1][start.1] ==Tile::N_E||tiles[start.0+1][start.1] ==Tile::N_W}{
        if startingpath_1 ==(0,0){
            startingpath_1 = (start.0 + 1, start.1)
        }else{
            startingpath_2 = (start.0 + 1, start.1)

        }
    }
    if start.1 > 0 && {tiles[start.0][start.1-1] == Tile::Hor||tiles[start.0][start.1-1] ==Tile::S_E||tiles[start.0][start.1-1] ==Tile::N_E}{
        if startingpath_1 ==(0,0){
            startingpath_1 = (start.0, start.1-1)
        }else{
            startingpath_2 = (start.0, start.1-1)
        }
    }
    if start.1 < tiles[0].len() && {tiles[start.0][start.1+1] == Tile::Hor||tiles[start.0][start.1+1] ==Tile::S_W||tiles[start.0][start.1+1] ==Tile::N_W}{
        startingpath_2 = (start.0, start.1+1)
    }
    let startingpath_1 = (startingpath_1.0 as isize, startingpath_1.1 as isize);
    let startingpath_2 = (startingpath_2.0 as isize, startingpath_2.1 as isize);
    //println!("{:?}", start);
    //println!("{:?}, {:?}", startingpath_1, startingpath_2);
    //println!("{:?}, {:?}", tiles[startingpath_1.0 as usize][startingpath_1.1 as usize].get_next((-1*start.0 as isize+startingpath_1.0, -1*start.1 as isize+ startingpath_1.1)), 
    tiles[startingpath_2.0 as usize][startingpath_2.1 as usize].get_next((-1*start.0 as isize+startingpath_2.0, -1*start.1 as isize+ startingpath_2.1));
    let mut last = (startingpath_1.0 - start.0 as isize, startingpath_1.1 - start.1 as isize);
    let mut current =({start.0 as isize + last.0} as usize, {start.1 as isize + last.1} as usize);
    
    //println!("current: {:?}, {:?}", current, last); 
    let mut list_corner = vec![vec![], vec![]];
    if (startingpath_1.0 + startingpath_2.0, startingpath_1.1 + startingpath_2.1) != (0,0){
        list_corner[0].push(start.0);
        list_corner[1].push(start.1)
    }
    println!("{:?}", start);
    let mut count = 1;
    loop{
        last = tiles[current.0][current.1].get_next(last);
        if tiles[current.0][current.1] == Tile::N_E|| tiles[current.0][current.1] == Tile::S_E||tiles[current.0][current.1] == Tile::S_W || tiles[current.0][current.1] == Tile::N_W{
            list_corner[0].push(current.0);
            list_corner[1].push(current.1)
        }
        current = ({current.0 as isize + last.0} as usize, {current.1 as isize + last.1} as usize);
        count+=1;
        //println!("here: {:?}, {:?}", last, current);
        if tiles[current.0][current.1] == Tile::Sta{
            break;
        }
    }
    if (startingpath_1.0 + startingpath_2.0, startingpath_1.1 + startingpath_2.1) != (0,0){
        list_corner[0].push(start.0);
        list_corner[1].push(start.1)
    }
    println!("{:?}", list_corner);
    let area = {
        let mut total = 0;
        for i in 0..list_corner[1].len()-1{
            total += list_corner[0][i]*list_corner[1][i+1];
            println!("{:?}",list_corner[0][i]*list_corner[1][i+1]);
        }
        total as isize
    }- {
        let mut total = 0;
        for i in 0..list_corner[1].len()-1{
            total += list_corner[0][i+1]*list_corner[1][i];
            println!("second: {:?}",list_corner[0][i+1]*list_corner[1][i]);
        }
        total as isize};
    println!("{count}");
    println!("area: {area}");
    let result = {(area)/2} - {count/2}+1;
    //while find_empty(&second_tiles) != (0,0){
    //    
    //    let empty = find_empty(&second_tiles);
    //    check_around(&mut second_tiles, empty, true);
    //    
    //
    //}
    //for i in second_tiles.clone(){
    //    println!("{:?}", i)
    //}
    //let mut count = 0;
    //for i in second_tiles{
    //    for i in i{
    //        if i == Tile::Inn{
    //            count += 1;
    //        }
    //    }
    //}
    return result
}

fn check_around(mut second_tiles:&mut Vec<Vec<Tile>>, tile: (usize, usize), mut bool: bool) -> bool{
        let around_key = [(-1,0), (0,1), (1,0), (0,-1), (-1, -1), (1,1), (-1, 1), (1,-1)];
        second_tiles[tile.0][tile.1] = Tile::Inn;
        if bool{
            for i in around_key{
                let around = get_tile(tile, i, &second_tiles);
                if around == Tile::Out{
                    bool = false
                }
            }
        }
        if !bool{
            second_tiles[tile.0][tile.1] = Tile::Out;
            for i in around_key{
                let around = get_tile(tile, i, &second_tiles);
                if around == Tile::Emp || around == Tile::Inn{
                    let coord = ({tile.0 as isize+i.0} as usize, {tile.1 as isize+i.1} as usize);
                    check_around(&mut second_tiles, coord, false);
                }
            }
        }
    return true
}

fn get_tile(coord: (usize, usize), delta: (isize, isize), tiles: &Vec<Vec<Tile>>) -> Tile{
    tiles[{coord.0 as isize+delta.0} as usize][{coord.1 as isize+delta.1} as usize]
}

fn find_empty(tiles: &Vec<Vec<Tile>>) -> (usize, usize){
    for (i, x) in tiles.iter().enumerate() {
        for (j, x) in x.iter().enumerate() {
            if *x == Tile::Emp{
                return (i, j)
            }
        }
    }
    return (0,0)
}