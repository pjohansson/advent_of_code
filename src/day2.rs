use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn read_file() -> String {
    let filename = "example_files/day2_input.txt";
    let path = Path::new(filename);

    let mut file = File::open(path).unwrap();
    let mut string = String::new();

    // Lesson learned: File access! Must be unwrapped.
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    };

    string
}

// Lesson learned: This is another way to pass an array (specifially Vec) to a function
fn min_array(array: Vec<i32>) -> i32 {
    // Lesson learned: `min` functions only on an iterator, and maybe not for floats
    match array.iter().min() {
        Some(&i) => i, // Lesson learned: `min` returns wrapped reference
        _        => 0,
    }
}

fn get_box_sides(dim_string: &str) -> Vec<i32> {
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

    sides
}

// Lesson learned: A Vec is returned like this
fn get_box_areas(dim_string: &str) -> Vec<i32> {
    let sides = get_box_sides(dim_string);

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

pub fn main(dim_boxes_str: &str) -> i32 {
    let mut area = 0;

    for line in dim_boxes_str.lines() {
        area += area_one_box(line);
    }

    area
}

fn area_one_box(dim_string: &str) -> i32 {
    let areas = get_box_areas(dim_string);
    // Lesson learned: `sum` on iterator is unstable
    let sum = areas[0] + areas[1] + areas[2];

    2*sum + min_array(areas)
}

pub fn extra(dim_boxes_str: &str) -> i32 {
    let mut length = 0;

    for line in dim_boxes_str.lines() {
        length += length_one_box(line);
    }

    length
}

fn length_one_box(dim_string: &str) -> i32 {
    let sides = get_box_sides(dim_string);

    let mut perimeters = Vec::new();
    for i in 0..3 {
        for j in i..3 {
            if i != j {
                perimeters.push(2*(sides[i] + sides[j]));
            }
        }
    }

    let volume = sides[0]*sides[1]*sides[2];
    volume + min_array(perimeters)
}

#[cfg(test)]
pub mod tests {
    // Lesson learned: Import with easy access from tests
    use day1::tests::{Expected, assert_expected_answers, assert_one_expected};
    use super::*;

    #[test]
    fn day2a() {
        let expected_answers = [
            Expected { input: "2x3x4",  result: 58 },
            Expected { input: "1x1x10", result: 43 },
            ];
        let expected_double = Expected { input: "2x3x4\n1x1x10\n", result: 58+43 };

        // Lesson learned: Access private functions in `super` module by explicit signature
        assert_expected_answers(&expected_answers, super::area_one_box);
        assert_one_expected(&expected_double, main);
   }

    #[test]
    fn day2b() {
        let expected_answers = [
            Expected { input: "2x3x4",  result: 34 },
            Expected { input: "1x1x10", result: 14 },
            ];
        let expected_double = Expected { input: "2x3x4\n1x1x10\n", result: 34+14 };

        assert_expected_answers(&expected_answers, super::length_one_box);
        assert_one_expected(&expected_double, extra);
    }
}

