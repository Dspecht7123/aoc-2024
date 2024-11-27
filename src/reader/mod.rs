use std::fs;

const PATH_TO_CRATE: &str = "/var/home/dspecht/Work/GitHub/aoc-2024";

pub fn read_puzzle_input(day: i8) -> String {
    let path_to_crate = PATH_TO_CRATE.to_owned();
    let path_to_input_file = &format!("/src/day{}/input.txt", day);
    let path = path_to_crate + path_to_input_file;
    fs::read_to_string(path).expect("Should have been able to read the file")
}