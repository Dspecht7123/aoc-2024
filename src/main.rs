use reader::read_puzzle_input;

pub mod day1;
pub mod reader;

const DAY: i8 = 1;

fn main() {
    match DAY {
        1 => day1::solve_day_1(read_puzzle_input(DAY)),
        _ => println!("day {} not solved yet :-(", DAY),
    }
}
