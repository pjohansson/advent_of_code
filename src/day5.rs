pub fn main(input: &String) -> usize {
    input.lines().filter(|&line| is_nice(line)).count()
}

pub fn extra(input: &String) -> usize {
    input.lines().filter(|&line| is_nice_new(line)).count()
}

fn is_nice(input: &str) -> bool {
    has_doublette_with_sep(input, 0) && has_num_wovels(input, 3) && !has_bad_string(input)
}

fn has_bad_string(input: &str) -> bool {
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

fn has_num_wovels(input: &str, num: usize) -> bool {
    // Lesson learned: Using `filter` to extract values from an iterator
    // Lesson learned: Pattern matching in closures
    input.chars().filter(|&c| is_wovel(c)).count() >= num
}

fn has_doublette_with_sep(input: &str, sep: usize) -> bool {
    let current = input.chars();

    // Lesson learned: How to create iterator that skips n values
    let ahead = input.chars().skip(sep+1);

    // Lesson learned: Zipping iterators
    // `zip` uses into_iterator which duplicates the input
    let iter = current.zip(ahead);

    // `zip` yields Some(a, b) while both iterators give values
    // could also use iterators and `by_ref` to borrow in for loop but ugly
    for (a, b) in iter {
        if a == b {
            return true;
        }
    }

    false
}

fn has_pair_without_overlap(input: &str) -> bool {
    let current = input.chars();
    let next = input.chars().skip(1);

    let iter = current.zip(next);

    // Lesson learned: `for` loops break at end of `zip` iterators
    // Obviously! No need to be too clever with pattern matching
    // if there is no need for something exotic
    for (i, (a, b)) in iter.enumerate() {
        let needle = &format!("{}{}", a, b);
        let (_, haystack) = input.split_at(i+2);

        if haystack.contains(needle) {
            return true;
        }
    }

    false
}

fn is_nice_new(input: &str) -> bool {
    has_doublette_with_sep(input, 1) && has_pair_without_overlap(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn doublette_no_sep() {
        assert_eq!(true,  super::has_doublette_with_sep("aab", 0));
        assert_eq!(false, super::has_doublette_with_sep("aba", 0));
        assert_eq!(true,  super::has_doublette_with_sep("abb", 0));
        assert_eq!(true,  super::has_doublette_with_sep("xyzzyyx", 0));
        assert_eq!(false, super::has_doublette_with_sep("xyzabc", 0));
    }

    #[test]
    fn bad_strings() {
        assert_eq!(false, super::has_bad_string("aaa"));
        assert_eq!(true,  super::has_bad_string("aaba"));
        assert_eq!(true,  super::has_bad_string("acdacd"));
        assert_eq!(true,  super::has_bad_string("pqaaa"));
        assert_eq!(true,  super::has_bad_string("aaaxy"));
    }

    #[test]
    fn three_wovels() {
        assert_eq!(true,  super::has_num_wovels("aaaa", 3));
        assert_eq!(true,  super::has_num_wovels("eee", 3));
        assert_eq!(true,  super::has_num_wovels("iii", 3));
        assert_eq!(true,  super::has_num_wovels("ooo", 3));
        assert_eq!(true,  super::has_num_wovels("uuuuuu", 3));
        assert_eq!(true,  super::has_num_wovels("aei", 3));
        assert_eq!(false, super::has_num_wovels("yyy", 3));
        assert_eq!(false, super::has_num_wovels("aey", 3));
    }

    #[test]
    fn nice_words() {
        assert_eq!(true,  super::is_nice("ugknbfddgicrmopn"));
        assert_eq!(true,  super::is_nice("aaa"));
        assert_eq!(false, super::is_nice("jchzalrnumimnmhp"));
        assert_eq!(false, super::is_nice("haegwjzuvuyypxyu"));
        assert_eq!(false, super::is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn day5a() {
        // Lesson learned: use `\` to ignore whitespace in static string creation`
        let string = "\
            ugknbfddgicrmopn\n\
            aaa\n\
            jchzalrnumimnmhp\n\
            haegwjzuvuyypxyu\n\
            dvszwmarrgswjxmb\n".to_string();

        assert_eq!(2, main(&string));
    }

    #[test]
    fn new_nice_words() {
        assert!(super::is_nice_new("qjhvhtzxzqqjkmpb"));
        assert!(super::is_nice_new("xxyxx"));
        assert!(!super::is_nice_new("uurcxstgmygtbstg"));
        assert!(!super::is_nice_new("ieodomkazucvgmuy"));
    }

    #[test]
    fn one_space_doublette() {
        assert_eq!(true,  super::has_doublette_with_sep("xyx", 1));
        assert_eq!(true,  super::has_doublette_with_sep("abcdefeghi", 1));
        assert_eq!(true,  super::has_doublette_with_sep("efe", 1));
        assert_eq!(true,  super::has_doublette_with_sep("aaa", 1));
        assert_eq!(false, super::has_doublette_with_sep("uurcxstgmygtbstg", 1));
    }

    #[test]
    fn pair_without_overlap() {
        assert_eq!(true,  super::has_pair_without_overlap("xyxy"));
        assert_eq!(true,  super::has_pair_without_overlap("aabcdefgaa"));
        assert_eq!(false, super::has_pair_without_overlap("aaa"));
    }

    #[test]
    fn day5b() {
        let string = "\
            qjhvhtzxzqqjkmpb\n\
            xxyxx\n\
            uurcxstgmygtbstg\n\
            ieodomkazucvgmuy\n".to_string();

        assert_eq!(2, extra(&string));
    }
}
