pub fn part_1(input: &str) -> usize{
    input.lines().map(|x|{
        let x = x.split(":").collect::<Vec<&str>>()[1];
        let x = x.split("|").collect::<Vec<&str>>();
        println!("{:?}", x);
        let winning = x[0].trim().split(' ').collect::<Vec<&str>>();
        let mut score = 0;
        for i in x[1].trim().split(' '){
            if i == "" {continue;}
            if winning.contains(&i){
                if score == 0{
                    score = 1;
                }else {
                    score *= 2;
                }
            }
        }
        score
    }).sum()
}

pub fn part_2(input: &str) -> usize{
    let scores = input.lines().map(|x|{
        let x = x.split(":").collect::<Vec<&str>>()[1];
        let x = x.split("|").collect::<Vec<&str>>();
        let winning = x[0].trim().split(' ').collect::<Vec<&str>>();
        let mut score:usize = 0;
        for i in x[1].trim().split(' '){
            if i == "" {continue;}
            if winning.contains(&i){
                score += 1
            }
        }
        score
    });  
    let mut multiplication:Vec<usize> = vec![1; 214];
    let mut final_score = 0;
    for (i, x) in scores.enumerate(){
        let mult = multiplication[i];
        println!("{}", x);
        final_score += mult;
        if i !=213{
            let final_index = if x+i > 213{213}else{x+i};
            for i in &mut multiplication[i+1..=final_index]{
                *i+= mult
            }
        }
        println!("{:?}", multiplication)
        
    }
    return final_score
}