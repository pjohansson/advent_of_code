extern crate md5;

pub fn main(input: &String) -> i32 {
    for i in 0.. {
        let string = input.clone() + &format!("{}", i);

        if get_hash(&string).starts_with("00000") {
            return i
        }
    }

    0
}

fn get_hash(input: &str) -> String {
    let mut context = md5::Context::new();
    context.consume(input.as_bytes());

    let mut digest = String::with_capacity(2 * 16);
    for x in &context.compute() {
        digest.push_str(&format!("{:02x}", x));
    }

    digest.to_string()
}

#[cfg(test)]
pub mod tests {
    use utils::tests::*;
    use super::*;
    extern crate md5;

    #[test]
    fn known_md5() {
        assert_eq!("000001dbbfa3a5c83a2d506429c7b00e".to_string(),
            super::get_hash("abcdef609043"));
    }

    #[test]
    fn day4a() {
        let expected_answers = [
            Expected { input: "abcdef",  result: 609043 },
            Expected { input: "pqrstuv", result: 1048970 },
            ];

        assert_all_expected(&expected_answers, main);
    }
}
