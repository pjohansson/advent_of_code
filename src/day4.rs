extern crate md5;
use std;

// How to know when it has failed?
pub fn main(input: &String, num_zeros: usize) -> Result<i32, &str> {
    // Lesson learned: How to set a variable number of zeros
    let needle = format!("{:0width$}", 0, width=num_zeros);

    for i in 0..std::i32::MAX {
        // Lesson learned: Creating formatted strings
        let string = format!("{}{}", input, i);

        if get_hash(&string).starts_with(&needle) {
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

    digest.to_string()
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

