use crate::helper;
use std::collections::HashMap;

const NUMBER_STRINGS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

pub fn part1() {
    let input = helper::read_file("src/day1/input.txt").unwrap();

    let mut result = Vec::new();
    
    for line in input.lines() {
        let mut line_chars = Vec::new();

        // push occurrence of allowed chars 
        for c in line.chars().filter(|&c| NUMBER_STRINGS.contains(&c.to_string().as_str())) {
            line_chars.push(c);
        }

        // get first and last chars and combine into string
        result.push(helper::chars_to_string(&first_and_last(line_chars)));
    }

    // convert strings to ints and sum
    let sum: usize = result.iter().map(|s| helper::string_to_int(&s)).sum();
    println!("day1:1 sum {}", sum);
}

fn first_and_last(input: Vec<char>) -> Vec<char> {
    match input.len() {
        0 => vec![],
        1 => vec![input[0], input[0]],
        _ => vec![input[0], input[input.len() - 1]],
    }
}

pub fn part2() {
    let input: String = helper::read_file("src/day1/input.txt").unwrap();

    let mut result = Vec::new();

    for line in input.lines() {
        let occurrence_map = helper::get_occurrences_of_search(line, &NUMBER_STRINGS);
        let (min_key, max_key) = find_min_max(occurrence_map);
        result.push(helper::combine_strings_as_ints(&min_key, &max_key));
    }

    let sum: usize = result.iter().sum();

    println!("day1:2 sum {}", sum);
}

fn find_min_max(map: HashMap<String, Vec<usize>>) -> (String, String) {
    let (mut min_key, mut min_val) = (String::new(), usize::MAX);
    let (mut max_key, mut max_val) = (String::new(), usize::MIN);

    for (key, value) in map.iter() {
        // find the min index this term occurred, use that if lowest so far
        if let Some(min) = value.iter().min() {
            if *min <= min_val {
                min_key = key.clone();
                min_val = *min;
            }
        }

        // find the max index this term occurred, use that if highest so far
        if let Some(max) = value.iter().max() {
            if *max >= max_val {
                max_key = key.clone();
                max_val = *max;
            }
        }
    }

    (min_key, max_key)
}