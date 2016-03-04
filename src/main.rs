mod day1;

fn main() {
    let day1_data = day1::get_data();
    println!("Day 1");
    println!("  Main: {}", day1::main(day1_data) );
    println!("  Extra: {}", day1::extra(day1_data) );
    println!("")
}

