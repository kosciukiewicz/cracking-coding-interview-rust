fn string_compression_1(s: &str) -> String {
    let mut s_chars = s.chars();
    let mut last_char = s_chars.next().unwrap();
    let mut last_char_count = 1;

    let mut result = String::new();
    result.push(last_char);

    for current_char in s_chars {
        if current_char == last_char {
            last_char_count += 1;
        } else {
            result.push_str(&last_char_count.to_string());
            last_char = current_char;
            last_char_count = 1;
            result.push(last_char);
        }
    }

    result.push_str(&last_char_count.to_string());

    if result.len() < s.len() {
        result
    } else {
        String::from(s)
    }
}

fn string_compression_2(s: &str) -> String {
    let mut s_chars = s.chars();
    let mut last_char = s_chars.next().unwrap();
    let mut last_char_count = 1;

    let compression_length = count_compression_length(s);
    if compression_length >= s.len() {
        return String::from(s);
    }

    let mut result = String::with_capacity(compression_length);
    result.push(last_char);

    for current_char in s_chars {
        if current_char == last_char {
            last_char_count += 1;
        } else {
            result.push_str(&last_char_count.to_string());
            last_char = current_char;
            last_char_count = 1;
            result.push(last_char);
        }
    }

    result.push_str(&last_char_count.to_string());

    result
}

fn count_compression_length(s: &str) -> usize {
    let mut s_chars = s.chars();
    let mut last_char = s_chars.next().unwrap();
    let mut last_char_count = 1;
    let mut compression_length = 1;

    for current_char in s_chars {
        if current_char == last_char {
            last_char_count += 1;
        } else {
            compression_length += last_char_count.to_string().len();
            last_char = current_char;
            last_char_count = 1;
            compression_length += 1;
        }
    }

    compression_length += last_char_count.to_string().len();

    compression_length
}

#[cfg(test)]
mod tests {
    use super::string_compression_1;

    use crate::string_compression_2;
    use rstest::rstest;

    #[rstest]
    #[case("Hello", "Hello")]
    #[case("aaabbbc", "a3b3c1")]
    #[case("aaaaaaaaaaabbccc", "a11b2c3")]
    fn test_string_compression_1(#[case] s1: String, #[case] expected: String) {
        assert_eq!(string_compression_1(&s1), expected);
    }

    #[rstest]
    #[case("Hello", "Hello")]
    #[case("aaabbbc", "a3b3c1")]
    #[case("aaaaaaaaaaabbccc", "a11b2c3")]
    fn test_string_compression_2(#[case] s1: String, #[case] expected: String) {
        assert_eq!(string_compression_2(&s1), expected);
    }
}

fn main() {
    println!("{}", string_compression_1("aabbbc"));
    println!("{}", string_compression_2("aabbbc"));
}
