use crate::helper::read_file;

const NUMBERS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
                            // "zero", "one", "two", "three", "four", "five",
                            // "six", "seven", "eight", "nine"];

pub fn part1() {
    println!("called `day1::part1()`");

    let input;

    match read_file("src/day1/input.txt") {
        Ok(contents) => {            
            input = contents;
        }
        Err(err) => {
            eprintln!("error with file: {}", err);
            return;
        }
    }

    println!("{}", input);


    let mut result = Vec::new();

    for line in input.lines(){
        let mut lineChars = Vec::new();
        
        for c in line.chars().filter(|&c| NUMBERS.contains(&c)) {
            lineChars.push(c);
        }

        result.push(
            chars_to_string(
                    &first_and_last(
                        lineChars)));
    }

    println!("lines {:?}", result);

    let ints = strings_to_ints(result).unwrap();
    let sum: i32 = ints.iter().sum();

    println!("sum {}", sum);
}

fn first_and_last (input: Vec<char>) -> Vec<char> {
    match input.len() {
        0 => Vec::new(),
        1 => { vec![input[0], input[0]] }
        _ => { vec![input[0], input[input.len() -1]] }
    }
}

fn chars_to_string(input: &[char]) -> String {
    input.iter().collect()
}

fn strings_to_ints(input: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut ints = Vec::new();
    for string in input {
        let parsed = string.parse::<i32>()?;
        ints.push(parsed);
    }

    Ok(ints)
}