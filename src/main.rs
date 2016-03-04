fn main() {
    println!("Hello, world!");
}

#[test]
fn test_day1a() {
    struct CheckResult<'a> {
        instructions: &'a str,
        result: i32
    };

    let expected = [
        CheckResult { instructions: "(())", result: 0 },
        CheckResult { instructions: "()()", result: 0 },
        CheckResult { instructions: "(((", result: 3 },
        CheckResult { instructions: "(()(()(", result: 3 },
        CheckResult { instructions: "))(((((", result: 3 },
        CheckResult { instructions: "())", result: -1 },
        CheckResult { instructions: "))(", result: -1 },
        CheckResult { instructions: ")))", result: -3 },
        CheckResult { instructions: ")())())", result: -3 },
    ];

    for check in &expected {
        assert!(day1_a(check.instructions) == check.result);
    }
}

fn day1_a(instructions: &str) -> i32 {
    let mut pos = 0;

    for c in instructions.chars() {
        pos += match c {
            '(' => 1,
            ')' => -1,
            _   => 0,
        }
    }

    pos
}
