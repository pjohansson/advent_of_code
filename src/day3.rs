// Lesson learned: To push a copy onto an array, implement the Copy and Clone traits
// Otherwise, the pushed copy will be a pointer which can not then be moved
#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn is_equal(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn update_position(&mut self, c: char) {
        match c {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _   => (),
        }
    }
}

// Lesson learned: Good practice to borrow ownership whenever possible
// otherwise the input is free'd after the function finishes
fn position_in_array(current: &Position, array: &Vec<Position>) -> bool {
    for other in array {
        if other.is_equal(current) {
            return true
        }
    }

    false
}

pub fn main(input: &String) -> i32 {
    let mut current = Position::new();
    let mut visited = Vec::new();
    visited.push(current);

    for c in input.chars() {
        current.update_position(c);

        if !position_in_array(&current, &visited) {
            visited.push(current);
        }
    }

    visited.len() as i32
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::tests::*;

    #[test]
    fn day3a() {
        let expected_answers = [
            Expected { input: ">",          result: 2 },
            Expected { input: "^>v<",       result: 4 },
            Expected { input: "^v^v^v^v^v", result: 2 }
            ];

        assert_all_expected(&expected_answers, main);
    }
}

