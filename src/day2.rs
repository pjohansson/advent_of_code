// Lesson learned: This is another way to pass an array (specifially Vec) to a function
fn min_array(array: Vec<i32>) -> i32 {
    // Lesson learned: `min` functions only on an iterator, and maybe not for floats
    match array.iter().min() {
        Some(&i) => i, // Lesson learned: `min` returns wrapped reference
        _        => 0,
    }
}

// Lesson learned: A Vec is returned like this
fn get_box_areas(dim_string: &str) -> Vec<i32> {
    // Lesson learned: `split` and `collect`!
    let array: Vec<&str> = dim_string.split('x').collect();
    let mut sides = Vec::new();

    // Lesson learned: How to convert strings to numbers via pattern matching
    for c in array {
        match c.parse::<i32>() {
            Ok(v) => sides.push(v),
            _     => (),
        }
    }

    // Lesson learned: How to construct vectors piece by piece
    let mut areas = Vec::new();

    for i in 0..3 {
        for j in i..3 {
            if i != j {
                areas.push(sides[i]*sides[j]);
            }
        }
    }

    areas
}

pub fn main(dim_string: &str) -> i32 {
    let areas = get_box_areas(dim_string);
    // Lesson learned: `sum` on iterator is unstable
    let sum = areas[0] + areas[1] + areas[2];

    2*sum + min_array(areas)
}

#[cfg(test)]
pub mod tests {
    // Lesson learned: Import with easy access from tests
    use day1::tests::{Expected,assert_expected_answers};
    use super::*;

    #[test]
    fn day2a() {
        let expected_answers = [
            Expected { input: "2x3x4",  result: 58 },
            Expected { input: "1x1x10", result: 43 },
            ];

        assert_expected_answers(&expected_answers, main);
    }
}

