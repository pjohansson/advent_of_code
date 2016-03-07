// Lesson learned: Modules are included internally like this
mod day1;
mod day2;
mod utils;

use utils::read_file;

fn main() {
    let day1_data = read_file("example_files/day1_input.txt");
    println!("Day 1");
    println!("  Main:  {}", day1::main(&day1_data));
    println!("  Extra: {}", day1::extra(&day1_data));
    println!("");

    let day2_data = read_file("example_files/day2_input.txt");
    println!("Day 2");
    println!("  Main:  {}", day2::main(&day2_data));
    println!("  Extra: {}", day2::extra(&day2_data));
    println!("");
}

