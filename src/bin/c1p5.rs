fn is_one_edit_away(s1: &str, s2: &str) -> bool {
    if s1.is_empty() || s2.is_empty() {
        return s1.len().abs_diff(s2.len()) <= 1;
    }

    if s1.len().abs_diff(s2.len()) <= 1 {
        check_one_difference(s1, s2)
    } else {
        false
    }
}

fn check_one_difference(s1: &str, s2: &str) -> bool {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };

    let mut shorter_index: usize = 0;
    let mut longer_index: usize = 0;
    let mut found_difference = false;
    while shorter_index < shorter.len() && longer_index < longer.len() {
        if shorter.chars().nth(shorter_index).unwrap() != longer.chars().nth(longer_index).unwrap()
        {
            if found_difference {
                return false;
            }
            found_difference = true;

            if shorter.len() == longer.len() {
                shorter_index += 1
            }
        } else {
            shorter_index += 1;
        }
        longer_index += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_one_edit_away;

    use rstest::rstest;

    #[rstest]
    #[case("Hello", "Helo", true)]
    #[case("Hello", "Heplo", true)]
    #[case("Hello", "Helllo", true)]
    #[case("Hello", "World", false)]
    #[case("Hello", "Worlds", false)]
    #[case("Hello", "Cats", false)]
    #[case("Hello", "", false)]
    #[case("H", "", true)]
    fn test_is_one_edit_away(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        assert_eq!(is_one_edit_away(&s1, &s2), expected);
    }
}

fn main() {
    println!("{}", is_one_edit_away("Hello", "Helo"));
}
