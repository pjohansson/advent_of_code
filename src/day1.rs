fn get_move(c: char) -> i32 {
    match c {
        '(' =>  1,
        ')' => -1,
        _   =>  0,
    }
}

// Lesson learned: `pub` keyword exports function
pub fn main(instructions: &String) -> i32 {
    let mut floor = 0;

    // Lesson learned: `chars` method yields proper characters
    for c in instructions.chars() {
        floor += get_move(c);
    }

    floor
}

pub fn extra(instructions: &String) -> i32 {
    let mut floor = 0;

    for (j, c) in instructions.chars().enumerate() {
        floor += get_move(c);

        // Lesson learned: Enumerate return usize, `as` keyword casts types
        match floor {
            -1 => return (j as i32) + 1,
            _  => (),
        }
    }

    -1 // Failure
}

#[cfg(test)]
pub mod tests {
    // Lesson learned: `super` references superset of module
    use utils::tests::*;
    use super::*;

    #[test]
    fn day1a() {

        // Lesson learned: Tuples are declared with (), Structs with {}
        let expected_answers = [
            Expected { input: "(())",    result:  0 },
            Expected { input: "()()",    result:  0 },
            Expected { input: "(((",     result:  3 },
            Expected { input: "(()(()(", result:  3 },
            Expected { input: "))(((((", result:  3 },
            Expected { input: "())",     result: -1 },
            Expected { input: "))(",     result: -1 },
            Expected { input: ")))",     result: -3 },
            Expected { input: ")())())", result: -3 },
            ];

        assert_all_expected(&expected_answers, main);
    }

    #[test]
    fn day1b () {
        let expected_answers = [
            Expected { input: ")",     result: 1 },
            Expected { input: "()())", result: 5 },
            ];

        assert_all_expected(&expected_answers, extra);
    }
}

