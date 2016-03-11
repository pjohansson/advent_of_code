pub fn main(input: &String) -> usize {
    input.lines().filter(|&line| is_nice(&line.to_string())).count()
}

fn is_nice(input: &String) -> bool {
    has_double_char(input) && has_num_wovels(input, 3) && !has_bad_string(input)
}

fn has_double_char(input: &String) -> bool {
    let mut iter = input.chars().peekable();

    // Loop over iterator and compare values against next
    // Lesson learned: Cannot `peek` in for loop since iterator is moved, not borrowed
    // Instead use a regular loop and handle variable assignment manually

    // Lesson learned: Can pattern match with `while let` since `next` returns Option
    while let Some(current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if next == current {
                return true
            }
        }
    }

    false
}

fn has_bad_string(input: &String) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];

    for string in bad_strings.iter() {
        if input.contains(string) {
            return true
        }
    }

    false
}

fn is_wovel(c: char) -> bool {
    // Lesson learned: Match several values at once
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _                           => false,
    }
}

fn has_num_wovels(input: &String, num: usize) -> bool {
    // Lesson learned: Using `filter` to extract values from an iterator
    // Lesson learned: Pattern matching in closures
    input.chars().filter(|&c| is_wovel(c)).count() >= num
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn double_chars() {
        assert_eq!(true,  super::has_double_char(&"aab".to_string()));
        assert_eq!(false, super::has_double_char(&"aba".to_string()));
        assert_eq!(true,  super::has_double_char(&"abb".to_string()));
        assert_eq!(true,  super::has_double_char(&"xyzzyyx".to_string()));
        assert_eq!(false, super::has_double_char(&"xyzabc".to_string()));
    }

    #[test]
    fn bad_strings() {
        assert_eq!(false, super::has_bad_string(&"aaa".to_string()));
        assert_eq!(true,  super::has_bad_string(&"aaba".to_string()));
        assert_eq!(true,  super::has_bad_string(&"acdacd".to_string()));
        assert_eq!(true,  super::has_bad_string(&"pqaaa".to_string()));
        assert_eq!(true,  super::has_bad_string(&"aaaxy".to_string()));
    }

    #[test]
    fn three_wovels() {
        assert_eq!(true,  super::has_num_wovels(&"aaaa".to_string(), 3));
        assert_eq!(true,  super::has_num_wovels(&"eee".to_string(), 3));
        assert_eq!(true,  super::has_num_wovels(&"iii".to_string(), 3));
        assert_eq!(true,  super::has_num_wovels(&"ooo".to_string(), 3));
        assert_eq!(true,  super::has_num_wovels(&"uuuuuu".to_string(), 3));
        assert_eq!(true,  super::has_num_wovels(&"aei".to_string(), 3));
        assert_eq!(false, super::has_num_wovels(&"yyy".to_string(), 3));
        assert_eq!(false, super::has_num_wovels(&"aey".to_string(), 3));
    }

    #[test]
    fn nice_words() {
        assert_eq!(true,  super::is_nice(&"ugknbfddgicrmopn".to_string()));
        assert_eq!(true,  super::is_nice(&"aaa".to_string()));
        assert_eq!(false, super::is_nice(&"jchzalrnumimnmhp".to_string()));
        assert_eq!(false, super::is_nice(&"haegwjzuvuyypxyu".to_string()));
        assert_eq!(false, super::is_nice(&"dvszwmarrgswjxmb".to_string()));
    }

    #[test]
    fn day5a() {
        let string = "\
            ugknbfddgicrmopn\n\
            aaa\n\
            jchzalrnumimnmhp\n\
            haegwjzuvuyypxyu\n\
            dvszwmarrgswjxmb\n".to_string();

        assert_eq!(2, main(&string));
    }
}
