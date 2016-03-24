// Lesson learned: Modules are included internally like this
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod utils;
use utils::read_file;

fn main() {
    if let Ok(day1_data) = read_file("example_files/day1_input.txt") {
        println!("Day 1");
        println!("  Main:  {}", day1::main(&day1_data));
        println!("  Extra: {}", day1::extra(&day1_data));
        println!("");
    }

    if let Ok(day2_data) = read_file("example_files/day2_input.txt") {
        println!("Day 2");
        println!("  Main:  {}", day2::main(&day2_data));
        println!("  Extra: {}", day2::extra(&day2_data));
        println!("");
    }

    if let Ok(day3_data) = read_file("example_files/day3_input.txt") {
        println!("Day 3");
        println!("  Main:  {}", day3::main(&day3_data));
        println!("  Extra: {}", day3::extra(&day3_data));
        println!("");
    }

    // Lesson learned: Conditional compilation based on compiler flags
    if cfg!(feature = "all") {
        if let Ok(day4_data) = read_file("example_files/day4_input.txt") {
            println!("Day 4");
            println!("  Main:  {}", day4::main(&day4_data, 5).unwrap());
            println!("  Extra: {}", day4::main(&day4_data, 6).unwrap());
            println!("");
        }
    }

    if let Ok(day5_data) = read_file("example_files/day5_input.txt") {
        println!("Day 5");
        println!("  Main:  {}", day5::main(&day5_data));
        println!("  Extra: {}", day5::extra(&day5_data));
        println!("");
    }

    if let Ok(day6_data) = read_file("example_files/day6_input.txt") {
        println!("Day 6");
        println!("  Main:  {}", day6::main(&day6_data, day6::Rule::Main));
        println!("  Extra: {}", day6::main(&day6_data, day6::Rule::Extra));
        println!("");
    }
}
