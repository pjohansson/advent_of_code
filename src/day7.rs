pub fn main(input: &str, wire: char) -> u16 {
    unimplemented!();
}

type Wire<'a> = &'a str;

// Operations are enums with input and output connections (wires)
#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation<'a> {
    Move(u16, Wire<'a>),
    And(Wire<'a>, Wire<'a>, Wire<'a>),
    Or(Wire<'a>, Wire<'a>, Wire<'a>),
    LShift(Wire<'a>, u16, Wire<'a>),
    RShift(Wire<'a>, u16, Wire<'a>),
    Not(Wire<'a>, Wire<'a>),
}

use self::Operation::*;

fn read_operation(input: &str) -> Result<Operation, &str> {
    for word in input.split_whitespace() {
        match word {
            "AND" => read_and_or(line)
            "OR" => println!("or"),
            "LSHIFT" => println!("lshift"),
            "RSHIFT" => println!("rshift"),
            "NOT" => println!("not"),
            _ => (),
        }
    }

    Err("Could not read operation.")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::Operation::*;

    #[test]
    fn list_of_commands() {
        let input = "123 -> x\n\
                    456 -> y\n\
                    x AND y -> d\n\
                    x OR y -> e\n\
                    x LSHIFT 2 -> f\n\
                    y RSHIFT 2 -> g\n\
                    NOT x -> h\n\
                    NOT y -> i";

        assert_eq!(72, main(input, 'd'));
        assert_eq!(507, main(input, 'e'));
        assert_eq!(492, main(input, 'f'));
        assert_eq!(114, main(input, 'g'));
        assert_eq!(65412, main(input, 'h'));
        assert_eq!(65079, main(input, 'i'));
        assert_eq!(123, main(input, 'x'));
        assert_eq!(456, main(input, 'y'));
    }

    #[test]
    fn read_operation() {
        assert_eq!(Ok(Move(123u16, "x")), super::read_operation("123 -> x"));
        assert_eq!(Ok(And("x", "y", "d")), super::read_operation("x AND y -> d"));
        assert_eq!(Ok(Or("x", "y", "e")), super::read_operation("x OR y -> e"));
        assert_eq!(Ok(LShift("x", 2, "f")), super::read_operation("x LSHIFT 2 -> f"));
        assert_eq!(Ok(RShift("y", 2, "g")), super::read_operation("y RSHIFT 2 -> g"));
        assert_eq!(Ok(Not("x", "h")), super::read_operation("NOT x -> h"));
        assert_eq!(Err("Could not read operation."), super::read_operation(""));
    }
}
