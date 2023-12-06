use crate::helper;
use std::collections::HashMap;

fn split_input(input: &str) -> (usize, Vec<&str>) {
    if let Some(index) = input.find(':') {
        let (left, game_content) = input.split_at(index + 1);

        let left_split: Vec<&str> = left.split(' ').collect();
        let left_num: String = left_split[1].chars().filter(|&c| c != ':').collect();

        (helper::string_to_int(left_num.trim()), game_content.trim().split(';').collect())
    } else {
        panic!("should not get here: {}", input)
    }
}

fn get_colour_count(colour: &str) -> (usize, &str) {
    let colour_count: Vec<&str> = colour.trim().split(' ').collect();
    (helper::string_to_int(colour_count[0].trim()), colour_count[1].trim())
}

fn check_validity(map: &HashMap<&str, usize>, allowed: &HashMap<&str, usize>) -> bool {
    for entry in map {
        let allowed = allowed.get(entry.0).unwrap();

        if entry.1 > allowed {
            return false;
        }
    }

    return true;
}

pub fn part1() {
    let input: String = helper::read_file("src/day2/input.txt").unwrap();
   
    let valid_format: HashMap<&str, usize> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]
    .iter()
    .cloned()
    .collect();

    let mut valid_sum = 0;

    for game in input.lines() {
        let rounds: (usize, Vec<&str>) = split_input(game);

        let mut is_valid = true;

        for round in rounds.1 {
            let mut map: HashMap<&str, usize> = HashMap::new();

            let colours: Vec<&str> = round.trim().split(',').collect();

            for colour in colours {
                let count: (usize, &str) = get_colour_count(colour);
                *map.entry(count.1).or_insert(0) += count.0;
            }

            if !check_validity(&map, &valid_format) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            valid_sum += rounds.0;
        }
    }

    println!("day2:1 result {}", valid_sum);
}

pub fn part2() {
    let input: String = helper::read_file("src/day2/input.txt").unwrap();

    let mut result = 0;

    for game in input.lines() {
        let rounds: (usize, Vec<&str>) = split_input(game);

        let mut map: HashMap<&str, usize> = HashMap::new();

        for round in rounds.1 {
            println!("round {}", round);
            let colours: Vec<&str> = round.trim().split(',').collect();

            // add colour value if lower
            for colour in colours {
                let count: (usize, &str) = get_colour_count(colour);

                if let Some(entry) = map.get_mut(count.1) {
                    if *entry < count.0 {
                        *entry = count.0;
                    }
                } else {
                    map.insert(count.1,count.0);
                }
            } 
        }
        println!("map {:?}", map);

        // get power of colours in game
        let mut power = 1;
        for entry in map.values() {
            power *= entry;
        }
        result += power;
    }

    println!("day2:2 result {}", result);
}
