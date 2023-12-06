use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

pub fn read_file(path: &str) -> Result<String, io::Error> {
    // println!("readTextFileToString: {}", path);

    let dir = env::current_dir()?;
    let full_path = dir.join(path);

    // println!("reading file: {}", full_path.to_string_lossy());

    fs::read_to_string(full_path)
}

pub fn string_to_int(string: &str) -> usize {
    let mappings: HashMap<&str, usize> = [
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

    match string.parse::<usize>() {
        Ok(num) => num,
        Err(_err) => *mappings
            .get(string)
            .unwrap_or_else(|| panic!("shouldn't get here... {}", string)),
    }
}

pub fn chars_to_string(input: &[char]) -> String {
    input.iter().collect()
}

pub fn combine_strings_as_ints(val1: &str, val2: &str) -> usize {
    let combined = format!("{}{}", string_to_int(val1), string_to_int(val2));
    string_to_int(&combined)
}

pub fn get_occurrences_of_search(
    input: &str,
    search_terms: &[&str],
) -> HashMap<String, Vec<usize>> {
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();

    // get start index for occurrences of match from search_terms
    for &search in search_terms.iter() {
        let occurrences: Vec<usize> = input
            .match_indices(search)
            .map(|(start, _)| start)
            .collect();

        if !occurrences.is_empty() {
            map.insert(search.to_string(), occurrences);
        }
    }

    map
}
