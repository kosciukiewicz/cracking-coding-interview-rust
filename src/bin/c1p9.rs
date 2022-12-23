struct IsSubstring {
    n_calls: usize,
}

impl IsSubstring {
    fn new() -> Self {
        IsSubstring { n_calls: 0 }
    }

    fn check(&mut self, s1: &str, s2: &str) -> bool {
        self.n_calls += 1;
        s1.contains(s2)
    }
}

fn is_rotation(s1: &str, s2: &str, is_substring_caller: &mut IsSubstring) -> bool {
    if s1.len() == s2.len() {
        let s1s1 = s1.to_owned() + s1;
        is_substring_caller.check(&s1s1, s2)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_rotation;

    use crate::IsSubstring;
    use rstest::rstest;

    #[rstest]
    #[case("Hello", "lloHe", true)]
    #[case("waterbottle", "ttlewaterbo", true)]
    #[case("waterbottle", "tlewaterbo", false)]
    #[case("Hello", "waterbottle", false)]
    fn test_is_rotation(#[case] s1: &str, #[case] s2: &str, #[case] expected: bool) {
        let mut is_substring = IsSubstring::new();

        assert_eq!(is_rotation(s1, s2, &mut is_substring), expected);
        assert!(is_substring.n_calls <= 1);
    }
}

fn main() {
    let mut is_substring = IsSubstring::new();
    println!(
        "{:?}",
        is_rotation("waterbottle", "ttlewaterbo", &mut is_substring),
    );
}
