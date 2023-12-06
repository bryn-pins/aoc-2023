use crate::helper::read_file;
use std::collections::HashMap;

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUMBER_STRINGS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

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

    let mut result = Vec::new();

    for line in input.lines() {
        let mut line_chars = Vec::new();

        for c in line.chars().filter(|&c| NUMBERS.contains(&c)) {
            line_chars.push(c);
        }

        result.push(chars_to_string(&first_and_last(line_chars)));
    }

    let mut ints = Vec::new();
    for string in result {
        ints.push(string_to_int(&string));
    }
    let sum: i32 = ints.iter().sum();

    println!("sum {}", sum);
}

pub fn part2() {
    println!("called `day1::part2()`");

    let input: String = read_file("src/day1/input.txt").unwrap();

    let mut result = Vec::new();

    for line in input.lines() {
        let mut map: HashMap<&str, Vec<usize>> = HashMap::new();

        for &search in NUMBER_STRINGS.iter() {
            let mut occurances = Vec::new();

            for (start, _) in line.match_indices(search) {
                let found = start;
                occurances.push(found);
            }

            if (occurances.len() > 0) {
                map.insert(search, occurances);
            }
        }

        let (mut min_key, mut min_val) = ("", usize::MAX);
        let (mut max_key, mut max_val) = ("", usize::MIN);

        for (key, value) in map.iter() {
            if let Some(min) = value.iter().min() {
                if *min <= min_val {
                    min_key = key;
                    min_val = *min;
                }
            }

            if let Some(max) = value.iter().max() {
                if *max >= max_val {
                    max_key = key;
                    max_val = *max;
                }
            }
        }

        let min = string_to_int(min_key);
        let max = string_to_int(max_key);
        let combined: String = min.to_string() + &max.to_string();
        result.push(string_to_int(&combined));
    }

    let sum: i32 = result.iter().sum();

    println!("sum {}", sum);
}

fn first_and_last(input: Vec<char>) -> Vec<char> {
    match input.len() {
        0 => Vec::new(),
        1 => {
            vec![input[0], input[0]]
        }
        _ => {
            vec![input[0], input[input.len() - 1]]
        }
    }
}

fn chars_to_string(input: &[char]) -> String {
    input.iter().collect()
}

fn string_to_int(string: &str) -> i32 {
    let spelling_to_num: HashMap<&str, i32> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    match string.parse::<i32>() {
        Ok(num) => num,
        Err(_err) => {
            let mut result = 0;

            match spelling_to_num.get(string) {
                Some(res) => {
                    result = *res;
                }
                None => {
                    panic!("shouldn't get here...")
                }
            }

            result
        }
    }
}
