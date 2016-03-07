mod day1;
mod day2;

fn main() {
    let day1_data = day1::get_data();
    println!("Day 1");
    println!("  Main: {}", day1::main(day1_data) );
    println!("  Extra: {}", day1::extra(day1_data) );
    println!("");

    let day2_data = day2::read_file();
    println!("Day 2");
    println!("  Main: {}", day2::main(&day2_data) );
    println!("  Extra: {}", day2::extra(&day2_data) );
    println!("");
}

