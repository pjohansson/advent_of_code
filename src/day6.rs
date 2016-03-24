// Lesson learned: Importing nested namespace methods from self
use self::Instruction::*;
use self::Rule::*;

pub fn main(input: &str, rule: Rule) -> usize {
    let mut lights = LightArray::new(1000, 1000);

    for (i, line) in input.lines().enumerate() {
        match (get_instruction(line), lights.slice_from_str(line)) {
            (Ok(instruction), Ok(slice)) => {
                for index in slice {
                    switch_light(&mut lights.array[index], instruction, rule);
                }
            },
            _ => println!("Could not parse line {} ({:?}).", i+1, line),
        }
    }

    lights.array.into_iter().fold(0, |acc, light| acc + light)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rule {
    Main,
    Extra
}

fn switch_light(light: &mut usize, instruction: Instruction, rule: Rule) {
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct LightArray {
    array: Vec<usize>,
    shape: Coordinate,
}

impl LightArray {
    fn new(x: usize, y: usize) -> LightArray {
        LightArray{array: vec!(0; x*y), shape: Coordinate(x, y)}
    }

    // Lesson learned: In this case the borrowed line must live 
    // for as long as the slicing function and it's results (I think)
    fn slice_from_str<'a>(&'a self, line: &'a str) -> Result<Slice, &str> {
        Slice::from_str(line, self.shape)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn from_str(input: &str) -> Result<Coordinate, &str> {
        let coords: Vec<usize> = input.split(",")
                                      .map(|cs| cs.parse::<usize>().unwrap())
                                      .take(2).collect();

        Ok(Coordinate(coords[0], coords[1]))
    }
}

struct Slice {
    begin: Coordinate,
    end: Coordinate,
    limits: Coordinate,
    current: Option<Coordinate>,
}

impl Slice {
    fn new(begin: Coordinate, end: Coordinate, limits: Coordinate) -> Slice {
        let current = begin;
        Slice {begin: begin, end: end, limits: limits, current: Some(current)}
    }

    fn from_str(input: &str, shape: Coordinate) -> Result<Slice, &str> {
        match get_coordinate_pairs(input) {
            Ok((begin, end)) => Ok(Slice::new(begin, end, shape)),
            Err(error)       => Err(error),
        }
    }

    fn index(&self, Coordinate(x, y): Coordinate) -> usize {
        let Coordinate(xmax, _) = self.limits;
        x + y*xmax
    }
}

// Lesson learned: Implementing an Iterator method for a struct
impl Iterator for Slice {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.current {
            None                   => None,
            Some(Coordinate(x, y)) => {
                let Coordinate(xmin, _) = self.begin;
                let Coordinate(xmax, ymax) = self.end;

                self.current = {
                    let (mut xnext, mut ynext) = (x + 1, y);

                    if xnext > xmax {
                        xnext = xmin;
                        ynext += 1;
                    }

                    if ynext <= ymax {
                        Some(Coordinate(xnext, ynext))
                    }
                    else {
                        None
                    }
                };

                Some(self.index(Coordinate(x, y)))
            },
        }
    }
}

fn get_coordinate_pairs(input: &str) -> Result<(Coordinate, Coordinate), &str> {
    let words: Vec<&str> = input.split_whitespace()
                                .collect();

    // Lesson learned: use `position` or `find` with a closure to search
    // through a Vec (as an iterator)
    let pos = words.iter().position(|&w| w == "through");
    
    if let Some((one, two)) = match pos {
        Some(i) => get_words(words, i-1, i+1),
        _       => None,
    } {
        if let (Ok(coord1), Ok(coord2)) = (Coordinate::from_str(one), Coordinate::from_str(two)) {
            return Ok((coord1, coord2));
        }
    }

    Err("Could not parse coordinates from input string.")
}

fn get_instruction(input: &str) -> Result<Instruction, &str> {
    let words: Vec<&str> = input.split_whitespace()
                                .take(2)
                                .collect();

    match get_words(words, 0, 1) {
        Some(("toggle", _))   => Ok(Toggle),
        Some(("turn", "on"))  => Ok(TurnOn),
        Some(("turn", "off")) => Ok(TurnOff),
        _                     => Err("Could not parse instruction from line."),
    }
}

fn get_words(array: Vec<&str>, pos1: usize, pos2: usize) -> Option<(&str, &str)> {
    // Lesson learned: use `get` to safely access elements of a Vec
    if let (Some(&w1), Some(&w2)) = (array.get(pos1), array.get(pos2)) {
        return Some((w1, w2));
    }

    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::{Coordinate, Slice};
    use super::Instruction::*;
    use super::Rule::*;

    #[test]
    fn parse_coordinate() {
        assert_eq!(Ok(Coordinate(0, 999)), Coordinate::from_str("0,999"));
        assert_eq!(Ok(Coordinate(5, 0)), Coordinate::from_str("5,0"));
    }

    #[test]
    fn parse_pair_of_coordinates() {
        assert_eq!(Ok((Coordinate(0,0), Coordinate(999,0))), 
            super::get_coordinate_pairs("turn on 0,0 through 999,0"));
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
        assert_eq!(Ok(Toggle), super::get_instruction("toggle 0,0 through 999,0"));
        assert_eq!(Ok(TurnOn), super::get_instruction("turn on 0,0 through 999,999"));
        assert_eq!(Ok(TurnOff), super::get_instruction("turn off 0,0 through 999,0"));
    }

    #[test]
    fn iterator() {
        let mut c0 = Coordinate(0, 0);
        let mut c1 = Coordinate(10, 0);
        let shape = Coordinate(1000, 1000);
        let (mut expected, mut result): (Vec<usize>, Vec<usize>);

        // A simple row slice
        expected = (0..11).collect();
        result = Slice::new(c0, c1, shape).collect();
        assert_eq!(expected, result);

        // A 11x2 box
        c1 = Coordinate(10, 1);
        expected = (0..11).chain(1000..1011).collect();
        result = Slice::new(c0, c1, shape).collect();
        assert_eq!(expected, result);

        // Change origins
        c0 = Coordinate(5, 1);
        c1 = Coordinate(10, 2);
        expected = (1005..1011).chain(2005..2011).collect();
        result = Slice::new(c0, c1, shape).collect();
        assert_eq!(expected, result);

        // Assert boundaries
        c0 = Coordinate(998, 998);
        c1 = Coordinate(999, 999);
        expected = (998998..999000).chain(999998..1000000).collect();
        result = Slice::new(c0, c1, shape).collect();
        assert_eq!(expected, result);
    }
    
    #[test]
    fn iterator_from_str() {
        let shape = Coordinate(1000, 1000);
        
        match Slice::from_str("toggle 0,0 through 10,1", shape) {
            Ok(slice) => {
                let expected: Vec<usize> = (0..11).chain(1000..1011).collect();
                let result: Vec<usize> = slice.collect();
                assert_eq!(expected, result);
            },
            Err(error)  => panic!(error),
        };
    }
}
