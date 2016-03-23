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
        let (c0, c1) = get_coordinate_pairs(&input).unwrap();
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rule {
    Main,
    Extra
}

// Lesson learned: Importing nested namespace methods from self
use self::Instruction::*;
use self::Rule::*;

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

pub fn main(input: &str, rule: Rule) -> i32 {
    let mut array: Vec<i32> = vec!(0; 1000*1000);

    for (i, line) in input.lines().enumerate() {
        if let Some(instruction) = get_instruction(line) {
            for index in Slice::from_str(&line) {
                switch_light(&mut array[index], instruction, rule);
            }
        }
        else {
            println!("Warning: Could not parse instruction in line {} ({:?})",
                     i, line);
        }
    }

    array.into_iter().fold(0, |acc, light| acc + light)
}

fn switch_light(light: &mut i32, instruction: Instruction, rule: Rule) {
    match rule {
        Main => match instruction {
            Toggle if *light == 1 => *light = 0,
            Toggle                => *light = 1,
            TurnOn                => *light = 1,
            TurnOff               => *light = 0,
        },
        Extra => match instruction {
            Toggle                => *light += 2,
            TurnOn                => *light += 1,
            TurnOff if *light > 0 => *light -= 1,
            _                     => (),
        }
    }
}

fn get_words(array: Vec<&str>, pos1: usize, pos2: usize) -> Option<(&str, &str)> {
    // Lesson learned: use `get` to safely access elements of a Vec
    if let (Some(&w1), Some(&w2)) = (array.get(pos1), array.get(pos2)) {
        return Some((w1, w2));
    }

    None
}

fn get_coordinate_pairs(input: &str) -> Option<(Coordinate, Coordinate)> {
    let words: Vec<&str> = input.split_whitespace()
                                .collect();

    // Lesson learned: use `position` or `find` with a closure to search
    // through a Vec (as an iterator)

    let pos = words.iter().position(|&w| w == "through");

    if let Some((one, two)) = match pos {
        Some(i) => get_words(words, i-1, i+1),
        _       => None,
    } {
        return Some((Coordinate::new(one), Coordinate::new(two)));
    }

    None
}

fn get_instruction(input: &str) -> Option<Instruction> {
    let words: Vec<&str> = input.split_whitespace()
                                .take(2)
                                .collect();

    match get_words(words, 0, 1) {
        Some(("toggle", _))   => Some(Toggle),
        Some(("turn", "on"))  => Some(TurnOn),
        Some(("turn", "off")) => Some(TurnOff),
        _                     => None,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::{Coordinate, Slice};
    use super::Instruction::*;
    use super::Rule::*;

    #[test]
    fn coordinate_pair() {
        assert_eq!(Coordinate(0, 999), Coordinate::new("0,999"));
        assert_eq!(Coordinate(5, 0), Coordinate::new("5,0"));
    }

    #[test]
    fn coordinate_pairs() {
        assert_eq!((Coordinate(0,0), Coordinate(999,0)), super::get_coordinate_pairs("turn on 0,0 through 999,0").unwrap());
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
        assert_eq!(1000000, main("turn on 0,0 through 999,999", Main));
        assert_eq!(1000, main("turn on 0,0 through 999,0", Main));
        assert_eq!(1000, main("toggle 0,0 through 999,0", Main));
        assert_eq!(0, main("turn off 0,0 through 999,0", Main));
    }

    #[test]
    fn instructions() {
        assert_eq!(Toggle, super::get_instruction("toggle 0,0 through 999,0").unwrap());
        assert_eq!(TurnOn, super::get_instruction("turn on 0,0 through 999,999").unwrap());
        assert_eq!(TurnOff, super::get_instruction("turn off 0,0 through 999,0").unwrap());
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
