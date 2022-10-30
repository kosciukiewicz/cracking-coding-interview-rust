use std::collections::HashMap;
use std::ops::BitXor;

fn is_palindrome_permutation_1(s: &str) -> bool {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    for c in s.to_lowercase().chars() {
        if c.is_whitespace() {
            continue;
        }

        if let Some(value) = char_map.get_mut(&c) {
            *value += 1;
        } else {
            char_map.insert(c, 1);
        }
    }

    let final_odd_count: usize =
        char_map.values().fold(
            0,
            |acc, char_count| {
                if char_count % 2 != 0 {
                    acc + 1
                } else {
                    acc
                }
            },
        );

    final_odd_count <= 1
}

fn is_palindrome_permutation_2(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let a_int_char: i64 = 'a' as i64;

    for c in s.to_lowercase().chars() {
        if c.is_whitespace() {
            continue;
        }

        let mut int_char: i64 = c as i64;
        int_char -= a_int_char;
        bitfield = bitfield.bitxor(1 << int_char);
    }

    bitfield.count_ones() <= 1
}

#[cfg(test)]
mod tests {
    use super::is_palindrome_permutation_1;

    use crate::is_palindrome_permutation_2;
    use rstest::rstest;

    #[rstest]
    #[case("Tact Coa", true)]
    #[case("Coa", false)]
    #[case("%a jak k%", true)]
    #[case("%%aakjj", true)]
    fn test_is_palindrome_permutation_1(#[case] s: String, #[case] expected: bool) {
        assert_eq!(is_palindrome_permutation_1(&s), expected);
    }

    #[rstest]
    #[case("Tact Coa", true)]
    #[case("Coa", false)]
    #[case("a jak k", true)]
    #[case("aakjj", true)]
    fn test_is_palindrome_permutation_2(#[case] s: String, #[case] expected: bool) {
        assert_eq!(is_palindrome_permutation_2(&s), expected);
    }
}

fn main() {
    println!("{}", is_palindrome_permutation_1("Tact Coa"));
    println!("{}", is_palindrome_permutation_2("Tact Coa"));
}
