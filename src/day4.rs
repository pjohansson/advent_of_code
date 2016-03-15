extern crate md5;

use std;
use std::thread;
use std::sync::mpsc;

const NTHREADS: i32 = 8;

// Lesson learned: returning Result and getting the max of a type
pub fn main(input: &String, num_zeros: usize) -> Result<i32, &str> {
    // Lesson learned: How to set a variable number of zeros
    let needle = format!("{:0width$}", 0, width=num_zeros);

    // Lesson learned: Sending results between threads over channels
    let (tx, rx) = mpsc::channel();
    let mut i = 0;

    while i < std::i32::MAX {
        // Every thread calculates a hash for one number and sends match
        for _ in 0..NTHREADS {
            let tx = tx.clone();
            let needle = needle.clone();

            // Lesson learned: Creating formatted strings
            let string = format!("{}{}", input, i);

            thread::spawn(move || {
                let result = if get_hash(&string).starts_with(&needle) {
                    Some(i)
                }
                else {
                    None
                };

                tx.send(result).unwrap();
            });

            i += 1;
        }

        let mut results = Vec::new();

        // After every loop, main thread retrieves any successes
        // and returns minimum, else continues
        for _ in 0..NTHREADS {
            if let Some(i) = rx.recv().unwrap() {
                results.push(i);
            }
        }

        if let Some(&i) = results.iter().min() {
            return Ok(i)
        }
    }

    Err("Could not find integer fulfilling hash requirements.")
}

// Lesson learned: use of the MD5 crate from tests
fn get_hash(input: &str) -> String {
    let mut context = md5::Context::new();
    context.consume(input.as_bytes());

    let mut digest = String::with_capacity(2 * 16);
    for b in &context.compute() {
        digest.push_str(&format!("{:02x}", b));
    }

    digest
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn known_md5() {
        assert_eq!("000001dbbfa3a5c83a2d506429c7b00e".to_string(),
            super::get_hash("abcdef609043"));
    }

    #[test]
    fn day4a() {
        assert_eq!(609043,  main(&"abcdef".to_string(), 5).unwrap());
        assert_eq!(1048970, main(&"pqrstuv".to_string(), 5).unwrap());
    }
}
