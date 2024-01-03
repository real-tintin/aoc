use std::collections::VecDeque;

const DIGIT_AS_STR_MAX_LEN: usize = 5; // E.g., "one" or "three".

pub fn part_1(input: std::path::PathBuf) -> u32 {
    solve(input, false)
}

pub fn part_2(input: std::path::PathBuf) -> u32 {
    solve(input, true)
}

fn solve(input: std::path::PathBuf, support_digits_as_strings: bool) -> u32 {
    let file = std::fs::read_to_string(input).unwrap();
    let lines = file.lines();

    let mut sum: u32 = 0;
    let mut deq: VecDeque<char> = VecDeque::new();

    for line in lines {
        deq.clear();

        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for r#char in line.chars() {
            if deq.len() == DIGIT_AS_STR_MAX_LEN {
                deq.pop_front();
            }
            deq.push_back(r#char);

            let mut parsed_digit = r#char.to_digit(10);

            if support_digits_as_strings && parsed_digit.is_none() {
                parsed_digit = str_to_digit(&deq.iter().collect::<String>());
            }

            if parsed_digit.is_none() {
                continue;
            }

            if first.is_none() {
                first = parsed_digit;
            } else {
                last = parsed_digit;
            }
        }

        if first.is_none() {
            log::warn!("No digits found in line: {}", line);
            continue;
        }

        if last.is_none() {
            last = first;
        }

        sum += first.unwrap() * 10 + last.unwrap();

        log::debug!("{}: {}{} : {}", line, first.unwrap(), last.unwrap(), sum);
    }

    sum
}

fn str_to_digit(as_str: &String) -> Option<u32> {
    match as_str.len() {
        3 => {
            if as_str == "one" {
                return Some(1);
            } else if as_str == "two" {
                return Some(2);
            } else if as_str == "six" {
                return Some(6);
            } else {
                return None;
            }
        }
        4 => {
            let as_str_three_last = &as_str[1..];

            if as_str_three_last == "one" {
                return Some(1);
            } else if as_str_three_last == "two" {
                return Some(2);
            } else if as_str_three_last == "six" {
                return Some(6);
            } else if as_str == "four" {
                return Some(4);
            } else if as_str == "five" {
                return Some(5);
            } else if as_str == "nine" {
                return Some(9);
            } else {
                return None;
            }
        }
        5 => {
            let as_str_three_last = &as_str[2..];
            let as_str_four_last = &as_str[1..];

            if as_str_three_last == "one" {
                return Some(1);
            } else if as_str_three_last == "two" {
                return Some(2);
            } else if as_str_three_last == "six" {
                return Some(6);
            } else if as_str_four_last == "four" {
                return Some(4);
            } else if as_str_four_last == "five" {
                return Some(5);
            } else if as_str_four_last == "nine" {
                return Some(9);
            } else if as_str == "three" {
                return Some(3);
            } else if as_str == "seven" {
                return Some(7);
            } else if as_str == "eight" {
                return Some(8);
            } else {
                return None;
            }
        }
        _ => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};
    use crate::utils::test_utils::get_input_path;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input_path(1)), 53080);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input_path(1)), 53268);
    }
}
