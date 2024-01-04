use core::panic;
use regex::Regex;
use std::vec::Vec;

use crate::types::Part;

const MAX_N_RED_CUBES: u32 = 12;
const MAX_N_GREEN_CUBES: u32 = 13;
const MAX_N_BLUE_CUBES: u32 = 14;

#[derive(Clone, Debug)]
struct Set {
    n_red_cubes: u32,
    n_green_cubes: u32,
    n_blue_cubes: u32,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
    is_possible: bool,
    power: u32,
}

impl Game {
    pub fn from_line(line: &str) -> Game {
        log::debug!("Parsing: {}", line);

        let mut game = Game {
            id: 0,
            sets: Vec::new(),
            is_possible: false,
            power: 0,
        };

        let re_game_id = Regex::new(r"Game ([0-9]+):").unwrap();
        let re_set = Regex::new(r"([0-9]+) (red|green|blue)(.+?|.*?)").unwrap();

        if let Some(caps) = re_game_id.captures(line) {
            game.id = caps[1].parse::<u32>().unwrap();

            log::debug!("Parsed game id: {}", game.id);
        } else {
            panic!("No game id found in line: {}", line);
        }

        let mut set = Set {
            n_red_cubes: 0,
            n_green_cubes: 0,
            n_blue_cubes: 0,
        };

        for (_, [n_cubes, color, del]) in re_set.captures_iter(line).map(|c| c.extract()) {
            match color {
                "red" => {
                    set.n_red_cubes += n_cubes.parse::<u32>().unwrap();
                }
                "green" => {
                    set.n_green_cubes += n_cubes.parse::<u32>().unwrap();
                }
                "blue" => {
                    set.n_blue_cubes += n_cubes.parse::<u32>().unwrap();
                }
                _ => panic!("Unknown color: {}", color),
            }

            match del {
                "," => continue,
                ";" | "" => {
                    game.sets.push(set.clone());

                    set.n_red_cubes = 0;
                    set.n_green_cubes = 0;
                    set.n_blue_cubes = 0;
                }
                _ => panic!("Unknown delimiter: {}", del),
            }

            log::debug!("Parsed set: {} {} {}", n_cubes, color, del);
        }

        game.compute_is_possible();
        game.compute_power();

        game
    }

    fn compute_is_possible(&mut self) {
        for set in &self.sets {
            if set.n_red_cubes > MAX_N_RED_CUBES {
                self.is_possible = false;
                return;
            }
            if set.n_green_cubes > MAX_N_GREEN_CUBES {
                self.is_possible = false;
                return;
            }
            if set.n_blue_cubes > MAX_N_BLUE_CUBES {
                self.is_possible = false;
                return;
            }
        }

        self.is_possible = true;
    }

    fn compute_power(&mut self) {
        let mut max_n_red_cubes = 0;
        let mut max_n_green_cubes = 0;
        let mut max_n_blue_cubes = 0;

        for set in &self.sets {
            if set.n_red_cubes > max_n_red_cubes {
                max_n_red_cubes = set.n_red_cubes;
            }
            if set.n_green_cubes > max_n_green_cubes {
                max_n_green_cubes = set.n_green_cubes;
            }
            if set.n_blue_cubes > max_n_blue_cubes {
                max_n_blue_cubes = set.n_blue_cubes;
            }
        }

        self.power = max_n_red_cubes * max_n_green_cubes * max_n_blue_cubes;
    }
}

pub fn solve(input: std::path::PathBuf, part: Part) -> u32 {
    let return_sum_of_powers = if part == Part::One { false } else { true };

    let file = std::fs::read_to_string(input).unwrap();
    let lines = file.lines();

    let mut sum_of_possible: u32 = 0;
    let mut sum_of_powers: u32 = 0;

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let game = Game::from_line(line);

        games.push(game.clone());

        if game.is_possible {
            sum_of_possible += game.id;
        }

        sum_of_powers += game.power;

        log::debug!("{:?}", game);
    }

    if return_sum_of_powers {
        return sum_of_powers;
    }

    sum_of_possible
}

#[cfg(test)]
mod tests {
    use super::{solve, Part};
    use crate::utils::test::get_input_path;

    #[test]
    fn solve_part_one() {
        assert_eq!(solve(get_input_path(2), Part::One), 2278);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(solve(get_input_path(2), Part::Two), 67953);
    }
}
