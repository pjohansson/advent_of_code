fn is_nice(input: &String) -> bool {
    has_double_char(input) && has_three_wovels(input) && !has_bad_string(input)
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
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _                           => false,
    }
}

fn has_three_wovels(input: &String) -> bool {
    let mut count = 0;

    for c in input.chars() {
        // Lesson learned: Match several values at once
        count += match is_wovel(c) {
            true  => 1,
            false => 0,
        }
    }

    if count >= 3 {
        return true
    }

    false
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
        assert_eq!(true,  super::has_three_wovels(&"aaaa".to_string()));
        assert_eq!(true,  super::has_three_wovels(&"eee".to_string()));
        assert_eq!(true,  super::has_three_wovels(&"iii".to_string()));
        assert_eq!(true,  super::has_three_wovels(&"ooo".to_string()));
        assert_eq!(true,  super::has_three_wovels(&"uuuuuu".to_string()));
        assert_eq!(true,  super::has_three_wovels(&"aei".to_string()));
        assert_eq!(false, super::has_three_wovels(&"yyy".to_string()));
        assert_eq!(false, super::has_three_wovels(&"aey".to_string()));
    }

    #[test]
    fn nice_words() {
        assert_eq!(true,  super::is_nice(&"ugknbfddgicrmopn".to_string()));
        assert_eq!(true,  super::is_nice(&"aaa".to_string()));
        assert_eq!(false, super::is_nice(&"jchzalrnumimnmhp".to_string()));
        assert_eq!(false, super::is_nice(&"haegwjzuvuyypxyu".to_string()));
        assert_eq!(false, super::is_nice(&"dvszwmarrgswjxmb".to_string()));
    }
}
