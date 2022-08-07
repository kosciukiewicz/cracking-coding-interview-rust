use std::collections::HashSet;

fn all_chars_unique_part_a(s: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for char in s.chars() {
        if set.contains(&char) {
            return false;
        }
        set.insert(char);
    }

    true
}

fn all_chars_unique_part_b(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let a_int_char: i32 = 'a' as i32;

    for char in s.chars() {
        let mut int_char: i32 = char as i32;
        int_char -= a_int_char;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        bitfield |= 1 << int_char;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{all_chars_unique_part_a, all_chars_unique_part_b};
    use rstest::rstest;

    #[rstest]
    #[case("Hello", false)]
    #[case("World", true)]
    #[case("we11", false)]
    #[case("ww12", false)]
    fn test_all_chars_unique_part_a(#[case] s: String, #[case] expected: bool) {
        assert_eq!(all_chars_unique_part_a(&s), expected);
    }

    #[rstest]
    #[case("hello", false)]
    #[case("world", true)]
    #[case("banana", false)]
    #[case("pear", true)]
    fn test_all_chars_unique_part_b(#[case] s: String, #[case] expected: bool) {
        assert_eq!(all_chars_unique_part_b(&s), expected);
    }
}

fn main() {
    println!("{}", all_chars_unique_part_a("Helloorld"));
    println!("{}", all_chars_unique_part_b("HelloWorld"));
}
