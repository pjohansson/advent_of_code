#[derive(Copy, Clone, Debug, PartialEq)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn new(input: &str) -> Coordinate {
        let coords: Vec<usize> = input.split(",")
                                      .map(|cs| cs.parse::<usize>().unwrap())
                                      .collect();

        Coordinate(coords[0], coords[1])
    }

    fn index(&self) -> usize {
        self.0 + self.1*1000
    }

    fn next(&self) -> Option<Coordinate> {
        let (mut x, mut y) = (self.0+1, self.1);

        if x >= 1000 {
            x = 0;
            y += 1;
        }

        if y >= 1000 {
            return None
        }

        Some(Coordinate(x, y))
    }
}

struct Slice {
    begin: Coordinate,
    end: Coordinate,
    current: Option<Coordinate>,
}

impl Slice {
    fn new(c0: Coordinate, c1: Coordinate) -> Slice {
        let current = Coordinate(c0.0, c0.1);

        Slice { begin: c0.clone(), end: c1.clone(), current: Some(current) }
    }

    fn from_str(input: &str) -> Slice {
        let (c0, c1) = get_coordinate_pairs(&input);
        Slice::new(c0, c1)
    }

    fn to_slice(&self, coord: Coordinate) -> Option<Coordinate> {
        // Get limits
        let Coordinate(x0, _)  = self.begin;
        let Coordinate(x1, y1) = self.end;
        let Coordinate(mut x, mut y) = coord;

        if x < x0 {
            x = x0;
        }
        else if x > x1 {
            x = x0;
            y += 1;
        }

        if y > y1 {
            return None
        }

        Some(Coordinate(x, y))
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

// Lesson learned: Implementing an Iterator method for a struct
impl Iterator for Slice {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        // Pattern match current position to ensure that it's available,
        // then ensure that it is within the limits and advance it
        if let Some(current) = self.current {
            if let Some(adjusted) = self.to_slice(current) {
                self.current = adjusted.next();

                return Some(adjusted.index())
            }
        }

        None
    }
}

pub fn main(input: &str) -> i32 {
    let mut light_array = vec!(false; 1000000);

    for line in input.lines() {
        let instruction = get_instruction(line).unwrap();

        for index in Slice::from_str(&line) {
            match instruction {
                Instruction::Toggle  => light_array[index] = !light_array[index],
                Instruction::TurnOn  => light_array[index] = true,
                Instruction::TurnOff => light_array[index] = false,
            }
        }
    }

    light_array.into_iter().filter(|&light| { light }).count() as i32
}

fn get_coordinate_pairs(input: &str) -> (Coordinate, Coordinate) {
    let words: Vec<&str> = input.split_whitespace().collect();

    // Lesson learned: use `position` or `find` with a closure to search
    // through a Vec (as an iterator)
    let pos = words.iter().position(|&word| word == "through").unwrap();

    // Lesson learned: use `get` to safely access elements of a Vec
    // Returns an Option which I don't care about here
    let c0 = Coordinate::new(words.get(pos-1).unwrap());
    let c1 = Coordinate::new(words.get(pos+1).unwrap());

    (c0, c1)
}

fn get_instruction(input: &str) -> Option<Instruction> {
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.get(0).unwrap() {
        &"toggle" => Some(Instruction::Toggle),
        _         => match words.get(1).unwrap() {
            &"on"  => Some(Instruction::TurnOn),
            &"off" => Some(Instruction::TurnOff),
            _      => None,
        },
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::{Coordinate, Slice, Instruction};

    #[test]
    fn coordinate_pair() {
        assert_eq!(Coordinate(0, 999), Coordinate::new("0,999"));
        assert_eq!(Coordinate(5, 0), Coordinate::new("5,0"));
    }

    #[test]
    fn coordinate_pairs() {
        assert_eq!((Coordinate(0,0), Coordinate(999,0)), super::get_coordinate_pairs("turn on 0,0 through 999,0"));
    }

    #[test]
    fn coordinate_index() {
        assert_eq!(0, Coordinate(0,0).index());
        assert_eq!(10, Coordinate(10,0).index());
        assert_eq!(1010, Coordinate(10,1).index());
    }

    #[test]
    fn coordinate_next() {
        assert_eq!(Some(Coordinate(1,0)), Coordinate(0,0).next());
        assert_eq!(Some(Coordinate(0,1)), Coordinate(999,0).next());
        assert_eq!(None, Coordinate(999,999).next());
    }

    #[test]
    fn single_commands() {
        assert_eq!(1000000, main("turn on 0,0 through 999,999"));
        assert_eq!(1000, main("turn on 0,0 through 999,0"));
        assert_eq!(1000, main("toggle 0,0 through 999,0"));
        assert_eq!(0, main("turn off 0,0 through 999,0"));
    }

    #[test]
    fn instructions() {
        assert_eq!(Instruction::Toggle, super::get_instruction("toggle 0,0 through 999,0").unwrap());
        assert_eq!(Instruction::TurnOn, super::get_instruction("turn on 0,0 through 999,999").unwrap());
        assert_eq!(Instruction::TurnOff, super::get_instruction("turn off 0,0 through 999,0").unwrap());
    }

    #[test]
    fn iterator() {
        let mut c0 = Coordinate(0,0);
        let mut c1 = Coordinate(10, 0);
        let (mut expected, mut result): (Vec<usize>, Vec<usize>);

        // A simple row slice
        expected = (0..11).collect();
        result = Slice::new(c0, c1).collect();
        assert_eq!(expected, result);

        // A 11x2 box
        c1 = Coordinate(10, 1);
        expected = (0..11).chain(1000..1011).collect();
        result = Slice::new(c0, c1).collect();
        assert_eq!(expected, result);

        // Change origins
        c0 = Coordinate(5, 1);
        c1 = Coordinate(10, 2);
        expected = (1005..1011).chain(2005..2011).collect();
        result = Slice::new(c0, c1).collect();
        assert_eq!(expected, result);

        // Assert boundaries
        c0 = Coordinate(998, 0);
        c1 = Coordinate(999, 1);
        expected = (998..1000).chain(1998..2000).collect();
        result = Slice::new(c0, c1).collect();
        assert_eq!(expected, result);

        c0 = Coordinate(998, 999);
        c1 = Coordinate(1020, 1020);
        expected = (999998..1000000).collect();
        result = Slice::new(c0, c1).collect();
        assert_eq!(expected, result);
    }
}
