pub fn part_1(input: &str) -> usize{
    input.lines().map(|i|{
        let numbers: Vec<char> = i.chars().filter(|x| x.is_ascii_digit()).collect();
        if numbers.len() >0 {
            {numbers.first().unwrap_or(&'0').to_digit(10).unwrap() *10 + numbers.last().unwrap_or(&'0').to_digit(10).unwrap()}.try_into().unwrap()
        }
        else{0}
    }).sum()
}

pub fn part_2(input: &str) -> usize{
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    input.lines().map(|i|{
        let mut digits:Vec<char> = vec![];
        let mut temporary = String::new();
        for x in i.chars(){
            if x.is_ascii_digit(){
                temporary = String::new();
                digits.push(x)
            }else{
                temporary += &x.to_string();
                for (index, text) in numbers.into_iter().enumerate(){
                    if temporary.contains(text){
                        digits.push(char::from_u32({index + 1} as u32).unwrap());
                        temporary = String::new()
                    }
                }
            }
        }
        println!("{:?}", digits);
        let number: usize = {digits.first().unwrap_or(&'0').to_digit(10).unwrap() *10 + digits.last().unwrap_or(&'0').to_digit(10).unwrap()}.try_into().unwrap();
        println!("{}: {}", i, number);
        number

    }).sum()
}