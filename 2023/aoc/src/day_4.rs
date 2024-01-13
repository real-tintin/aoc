use regex::Regex;
use std::{borrow::BorrowMut, iter::zip};

use crate::types::Part;

const WINNING_CARDS_SIZE: usize = 10;
const DEALT_CARDS_SIZE: usize = 25;
const N_CARDS: usize = 187;

type WinningCards = [u32; WINNING_CARDS_SIZE];
type DealtCards = [u32; DEALT_CARDS_SIZE];

type WinningCards2d = [WinningCards; N_CARDS];
type DealtCards2d = [DealtCards; N_CARDS];

pub fn solve(input: std::path::PathBuf, part: Part) -> u32 {
    let file = std::fs::read_to_string(input).unwrap();
    let lines = file.lines();

    let mut sum: u32 = 0;
    let (winning_cards_2d, dealt_cards_2d) = parse_lines(lines);

    if part == Part::One {
        part_one(&winning_cards_2d, &dealt_cards_2d, sum.borrow_mut());
    } else {
        part_two(&winning_cards_2d, &dealt_cards_2d, sum.borrow_mut());
    }

    sum
}

fn parse_lines(lines: std::str::Lines) -> (WinningCards2d, DealtCards2d) {
    let mut winning_cards_2d = [[0_u32; WINNING_CARDS_SIZE]; N_CARDS];
    let mut dealt_cards_2d = [[0_u32; DEALT_CARDS_SIZE]; N_CARDS];

    let re = Regex::new(r"\d+").unwrap();

    for (i_line, line) in lines.enumerate() {
        let mut i_winning_card: usize = 0;
        let mut i_our_card: usize = 0;

        for (i_cap, cap) in re.captures_iter(line).enumerate() {
            if i_cap == 0 {
                continue;
            }

            let as_str = cap.get(0).unwrap().as_str();

            match as_str.parse::<u32>() {
                Ok(number) => {
                    if i_winning_card < WINNING_CARDS_SIZE {
                        winning_cards_2d[i_line][i_winning_card] = number;
                        i_winning_card += 1;
                    } else if i_our_card < DEALT_CARDS_SIZE {
                        dealt_cards_2d[i_line][i_our_card] = number;
                        i_our_card += 1;
                    } else {
                        panic!("Dimension mismatch")
                    }
                }
                Err(_) => panic!("Unable to parse expected number"),
            }
        }
    }

    (winning_cards_2d, dealt_cards_2d)
}

fn part_one(winning_cards_2d: &WinningCards2d, dealt_cards_2d: &DealtCards2d, sum: &mut u32) {
    let mut points: u32;

    for (i_card, (winning_cards, dealt_cards)) in zip(winning_cards_2d, dealt_cards_2d).enumerate()
    {
        match get_number_of_winning_numbers(winning_cards, dealt_cards) {
            0 => points = 0,
            x => points = 2_u32.pow(x - 1),
        }

        log::debug!("Card {} yields points: {}", i_card, points);

        *sum += points;
    }
}

fn part_two(winning_cards_2d: &WinningCards2d, dealt_cards_2d: &DealtCards2d, sum: &mut u32) {
    *sum += N_CARDS as u32;

    for i_card in 0..N_CARDS {
        let mut recursive_sum: u32 = 0;

        get_recursive_sum_of_copies(
            winning_cards_2d,
            dealt_cards_2d,
            recursive_sum.borrow_mut(),
            i_card,
        );

        log::debug!("Card {} yields recursive sum: {}", i_card, recursive_sum);

        *sum += recursive_sum
    }
}

fn get_recursive_sum_of_copies(
    winning_cards_2d: &WinningCards2d,
    dealt_cards_2d: &DealtCards2d,
    sum: &mut u32,
    i_start_card: usize,
) {
    let n_wins = get_number_of_winning_numbers(
        &winning_cards_2d[i_start_card],
        &dealt_cards_2d[i_start_card],
    ) as usize;

    for i_card in (i_start_card + 1)..(i_start_card + 1 + n_wins) {
        get_recursive_sum_of_copies(winning_cards_2d, dealt_cards_2d, sum, i_card)
    }

    *sum += n_wins as u32;
}

fn get_number_of_winning_numbers(winning_cards: &WinningCards, dealt_cards: &DealtCards) -> u32 {
    let mut n_wins: u32 = 0;

    for winning_card in winning_cards {
        for dealt_card in dealt_cards {
            if winning_card == dealt_card {
                n_wins += 1;
            }
        }
    }

    n_wins
}

#[cfg(test)]
mod tests {
    use super::{solve, Part};
    use crate::utils::test::get_input_path;

    #[test]
    fn solve_part_one() {
        assert_eq!(solve(get_input_path(4), Part::One), 22674);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(solve(get_input_path(4), Part::Two), 5747443);
    }
}
