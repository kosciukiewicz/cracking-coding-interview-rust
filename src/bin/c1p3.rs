fn make_url_1(s: &str) -> String {
    s.trim().replace(' ', "%20")
}

fn make_url_2(s: &str) -> String {
    s.split_whitespace().fold(String::new(), |acc, substring| {
        println!("s {}", substring);
        if acc.is_empty() {
            acc + substring
        } else {
            acc + "%20" + substring
        }
    })
}

#[cfg(test)]
mod tests {
    use super::make_url_1;
    use crate::make_url_2;

    use rstest::rstest;

    #[rstest]
    #[case("Hello", "Hello")]
    #[case("Hel lo ", "Hel%20lo")]
    fn test_make_url_1(#[case] s: String, #[case] expected: String) {
        assert_eq!(make_url_1(&s), expected);
    }

    #[rstest]
    #[case("Hello", "Hello")]
    #[case("Hel lo ", "Hel%20lo")]
    fn test_make_url_2(#[case] s: String, #[case] expected: String) {
        assert_eq!(make_url_2(&s), expected);
    }
}

fn main() {
    println!("{}", make_url_1("Hel lo "));
    println!("{}", make_url_2("Hel lo "));
}
