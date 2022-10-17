use std::collections::HashMap;

fn is_permutation(s2: &str, s1: &str) -> bool {
    let mut char_map: HashMap<char, i8> = HashMap::new();

    for char in s1.chars() {
        if let Some(value) = char_map.get_mut(&char) {
            *value += 1;
        } else {
            char_map.insert(char, 1);
        }
    }

    for char in s2.chars() {
        if let Some(value) = char_map.get_mut(&char) {
            *value -= 1;

            if *value < 0 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_permutation;
    use rstest::rstest;

    #[rstest]
    #[case("Hello", "Hello", true)]
    #[case("Hello", "Hllo", false)]
    #[case("Hello", "Hlloe", true)]
    fn test_is_permutation(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        assert_eq!(is_permutation(&s1, &s2), expected);
    }
}

fn main() {
    println!("{}", is_permutation("Hello", "olleH"));
}
