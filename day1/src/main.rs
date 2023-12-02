use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = std::fs::read_to_string("input")?;
    let result_a: u32 = parse_numbers(&input, false).iter().sum();
    println!("{}", result_a);
    let result_b: u32 = parse_numbers(&input, true).iter().sum();
    println!("{}", result_b);

    Ok(())
}

fn parse_numbers(input: &str, convert_text_to_digits: bool) -> Vec<u32> {
    fn get_number(line: &str) -> Option<u32> {
        match extract_first_and_last_number(line) {
            (Some(n1), Some(n2)) => Some(n1 * 10 + n2),
            (_, _) => None,
        }
    }
    input
        .lines()
        .filter_map(|line| {
            if convert_text_to_digits {
                get_number(replace_first_and_last_number_strings_with_digits(line).as_str())
            } else {
                get_number(line)
            }
        })
        .collect()
}

fn extract_first_and_last_number(string_with_digits: &str) -> (Option<u32>, Option<u32>) {
    let all_digits: Vec<u32> = string_with_digits
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    (all_digits.first().copied(), all_digits.last().copied())
}

fn replace_first_and_last_number_strings_with_digits(input: &str) -> String {
    const ONE: &str = "one";
    const TWO: &str = "two";
    const THREE: &str = "three";
    const FOUR: &str = "four";
    const FIVE: &str = "five";
    const SIX: &str = "six";
    const SEVEN: &str = "seven";
    const EIGHT: &str = "eight";
    const NINE: &str = "nine";

    type R<'a> = (&'a str, &'a str, Option<usize>, Option<usize>);
    let positions: Vec<R> = vec![
        (ONE, "1", input.find(ONE), input.rfind(ONE)),
        (TWO, "2", input.find(TWO), input.rfind(TWO)),
        (THREE, "3", input.find(THREE), input.rfind(THREE)),
        (FOUR, "4", input.find(FOUR), input.rfind(FOUR)),
        (FIVE, "5", input.find(FIVE), input.rfind(FIVE)),
        (SIX, "6", input.find(SIX), input.rfind(SIX)),
        (SEVEN, "7", input.find(SEVEN), input.rfind(SEVEN)),
        (EIGHT, "8", input.find(EIGHT), input.rfind(EIGHT)),
        (NINE, "9", input.find(NINE), input.rfind(NINE)),
    ];

    fn compare_min(x: &R, y: &R) -> std::cmp::Ordering {
        match (x, y) {
            ((_, _, Some(pos1), _), (_, _, Some(pos2), _)) => pos1.cmp(pos2),
            ((_, _, Some(_), _), (_, _, None, _)) => std::cmp::Ordering::Less,
            ((_, _, None, _), (_, _, Some(_), _)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn compare_max(x: &R, y: &R) -> std::cmp::Ordering {
        match (x, y) {
            ((_, _, _, Some(pos1)), (_, _, _, Some(pos2))) => pos1.cmp(pos2),
            ((_, _, _, Some(_)), (_, _, _, None)) => std::cmp::Ordering::Greater,
            ((_, _, _, None), (_, _ , _, Some(_))) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }

    let maybe_first = positions.iter().min_by(|x, y| compare_min(x, y));
    let maybe_last = positions.iter().max_by(|x, y| compare_max(x, y));

    if let Some(first) = maybe_first {
        let first_replaced = String::from(input).replacen(first.0, first.1, 1);
        if let Some(last) = maybe_last {
            return first_replaced.replace(last.0, last.1);
        }
        return first_replaced;
    }
    input.into()
}

#[cfg(test)]
mod tests {
    use crate::parse_numbers;

    fn input_a() -> String {
        String::from(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        )
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers(&input_a(), false), vec![12, 38, 15, 77]);
        assert_eq!(parse_numbers(&input_a(), false).iter().sum::<u32>(), 142);
    }

    fn input_b() -> String {
        String::from(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteent",
        )
    }
    #[test]
    fn test_parse_text_numbers() {
        assert_eq!(
            parse_numbers(&input_b(), true),
            vec![29, 83, 13, 24, 42, 14, 76]
        );
        assert_eq!(parse_numbers(&input_b(), false).iter().sum::<u32>(), 209);
    }

    fn input_c() -> String {
        String::from(
            "eightoneight234eighthree
            eightonethreeight234eightone",
        )
    }
    #[test]
    fn test_parse_text_numbers_edge_cases() {
        assert_eq!(
            parse_numbers(&input_c(), true),
            vec![83, 81]
        );
    }

}
