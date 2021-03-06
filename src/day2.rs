struct Rectangle {
    x: i32,
    y: i32,
    z: i32
}

// Lesson learned: How to implement methods
impl Rectangle {
    fn new(input: &str) -> Rectangle {
        let mut sides = Vec::new();

        let input_split: Vec<&str> = input.split('x').collect();

        for cs in input_split {
            // Lesson learned: `unwrap` returns Ok(result) from an `Option`
            sides.push(cs.parse::<i32>().unwrap())
        }

        Rectangle { x: sides[0], y: sides[1], z: sides[2] }
    }

    fn area(&self) -> i32 {
        2*(self.x * self.y + self.x * self.z + self.y * self.z)
    }

    fn get_areas(&self) -> [i32; 3] {
        [ self.x * self.y, self.x * self.z, self.y * self.z ]
    }

    fn get_perimeters(&self) -> [i32; 3] {
        [ 2*(self.x + self.y), 2*(self.x + self.z), 2*(self.y + self.z) ]
    }

    fn volume(&self) -> i32 {
        self.x * self.y * self.z
    }
}

// Lesson learned: This is another way to pass an array (specifially Vec) to a function
fn min_array(array: [i32; 3]) -> i32 {
    // Lesson learned: `min` functions only on an iterator, and maybe not for floats
    match array.iter().min() {
        Some(&i) => i, // Lesson learned: `min` returns wrapped reference
        _        => 0,
    }
}

pub fn main(input: &String) -> i32 {
    // Lesson learned: `sum` still not stable which is totally weird
    // input.lines().map(|line| area_one_box(&line.to_string())).sum()
    // Lesson learned: using a `fold` to accumulate values instead of `sum`
    input.lines().fold(0, |acc, line| acc + wrap_area(&line.to_string()))
}

fn wrap_area(dim_string: &String) -> i32 {
    let rectangle = Rectangle::new(dim_string);
    rectangle.area() + min_array(rectangle.get_areas())
}

pub fn extra(input: &String) -> i32 {
    input.lines().fold(0, |acc, line| acc + ribbon_length(&line.to_string()))
}

fn ribbon_length(dim_string: &String) -> i32 {
    let rectangle = Rectangle::new(dim_string);
    rectangle.volume() + min_array(rectangle.get_perimeters())
}

#[cfg(test)]
pub mod tests {
    // Lesson learned: Import with easy access from tests
    use utils::tests::*;
    use super::*;

    #[test]
    fn day2a() {
        let expected_answers = [
            Expected { input: "2x3x4",  result: 58 },
            Expected { input: "1x1x10", result: 43 },
            ];
        let expected_double = Expected { input: "2x3x4\n1x1x10\n", result: 58+43 };

        // Lesson learned: Access private functions in `super` module by explicit signature
        assert_all_expected(&expected_answers, super::wrap_area);
        assert_one_expected(&expected_double, main);
   }

    #[test]
    fn day2b() {
        let expected_answers = [
            Expected { input: "2x3x4",  result: 34 },
            Expected { input: "1x1x10", result: 14 },
            ];
        let expected_double = Expected { input: "2x3x4\n1x1x10\n", result: 34+14 };

        assert_all_expected(&expected_answers, super::ribbon_length);
        assert_one_expected(&expected_double, extra);
    }
}
