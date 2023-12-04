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
    let number_keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
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
                        digits.push(number_keys[index]);
                        temporary = String::new();
                        break
                    }
                }
            }
        }
        println!("{:?}", digits);
        let number = if digits.len() > 1{
            {digits.first().unwrap_or(&'0').to_digit(10).unwrap() *10 + digits.last().unwrap_or(&'0').to_digit(10).unwrap()}.try_into().unwrap()
        }else if digits.len() == 1{
            {digits.first().unwrap_or(&'0').to_digit(10).unwrap() *10}.try_into().unwrap()
        }else{
            0
        };
        println!("{}: {}", i, number);
        number

    }).sum()
}