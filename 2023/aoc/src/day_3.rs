use regex::Regex;
use std::str::Lines;
use std::vec::Vec;

use crate::types::Part;

const INPUT_TYPE_NONE: i32 = -1;
const INPUT_TYPE_SYMBOL_GEAR: i32 = -2;
const INPUT_TYPE_SYMBOL_OTHER: i32 = -3;

struct Symbol {
    edges: Vec<(usize, usize)>,
    input_type: i32,
}

struct Schematic {
    input: Vec<Vec<i32>>,
    numbers: Vec<u32>,
    symbols: Vec<Symbol>,

    part_numbers: Vec<u32>,
    gear_ratios: Vec<u32>,
}

impl Schematic {
    pub fn from_lines(lines: Lines) -> Schematic {
        let mut schematic = Schematic {
            input: Vec::new(),
            numbers: Vec::new(),
            symbols: Vec::new(),

            part_numbers: Vec::new(),
            gear_ratios: Vec::new(),
        };

        schematic.build_input(lines);
        schematic.build_symbols();

        schematic.find_part_numbers();
        schematic.find_gear_ratios();

        schematic
    }

    fn is_number(value: &i32) -> bool {
        value > &INPUT_TYPE_NONE
    }

    fn is_symbol(value: &i32) -> bool {
        value <= &INPUT_TYPE_SYMBOL_GEAR
    }

    fn is_gear(value: &i32) -> bool {
        value == &INPUT_TYPE_SYMBOL_GEAR
    }

    fn build_input(&mut self, lines: Lines) {
        let re = Regex::new(r"(\d+|.)").unwrap();

        for line in lines {
            log::debug!("Parsing: {}", line);

            let mut i_col = 0;
            let mut row = vec![INPUT_TYPE_NONE; line.len()];

            for cap in re.captures_iter(line) {
                let as_str = cap.get(0).unwrap().as_str();

                match as_str.parse::<u32>() {
                    Ok(number) => {
                        for element in row.iter_mut().skip(i_col).take(as_str.len()) {
                            *element = self.numbers.len() as i32;
                        }

                        self.numbers.push(number);
                    }
                    Err(_) => match as_str {
                        "." => row[i_col] = INPUT_TYPE_NONE,
                        "*" => row[i_col] = INPUT_TYPE_SYMBOL_GEAR,
                        _ => row[i_col] = INPUT_TYPE_SYMBOL_OTHER,
                    },
                }

                i_col += as_str.len();
            }

            self.input.push(row);
        }
    }

    fn build_symbols(&mut self) {
        for (i_row, row) in self.input.iter().enumerate() {
            for (i_col, _) in row.iter().enumerate() {
                if !Schematic::is_symbol(&self.input[i_row][i_col]) {
                    continue;
                }

                let mut symbol = Symbol {
                    edges: Vec::new(),
                    input_type: self.input[i_row][i_col],
                };

                let mut potential_edges: Vec<(usize, usize)> = Vec::new();
                let mut number_ids: Vec<i32> = Vec::new();

                // Top
                if i_row > 0 {
                    potential_edges.push((i_row - 1, i_col));
                }

                // Bottom
                if i_row < self.input.len() - 1 {
                    potential_edges.push((i_row + 1, i_col));
                }

                // Left
                if i_col > 0 {
                    potential_edges.push((i_row, i_col - 1));
                }

                // Right
                if i_col < row.len() - 1 {
                    potential_edges.push((i_row, i_col + 1));
                }

                // Top Left
                if i_row > 0 && i_col > 0 {
                    potential_edges.push((i_row - 1, i_col - 1));
                }

                // Top Right
                if i_row > 0 && i_col < row.len() - 1 {
                    potential_edges.push((i_row - 1, i_col + 1));
                }

                // Bottom Left
                if i_row < self.input.len() - 1 && i_col > 0 {
                    potential_edges.push((i_row + 1, i_col - 1));
                }

                // Bottom Right
                if i_row < self.input.len() - 1 && i_col < row.len() - 1 {
                    potential_edges.push((i_row + 1, i_col + 1));
                }

                for edge in potential_edges {
                    let value = self.input[edge.0][edge.1];

                    if Schematic::is_number(&value) && !number_ids.contains(&value) {
                        symbol.edges.push(edge);
                        number_ids.push(value);
                    }
                }

                self.symbols.push(symbol);
            }
        }
    }

    fn find_part_numbers(&mut self) {
        for symbol in &self.symbols {
            for edge in &symbol.edges {
                let part_number = self.numbers[self.input[edge.0][edge.1] as usize];

                self.part_numbers.push(part_number);

                log::debug!("Found part number: {}", part_number);
            }
        }
    }

    fn find_gear_ratios(&mut self) {
        for symbol in &self.symbols {
            if Schematic::is_gear(&symbol.input_type) && symbol.edges.len() == 2 {
                let gear_0 =
                    self.numbers[self.input[symbol.edges[0].0][symbol.edges[0].1] as usize];
                let gear_1 =
                    self.numbers[self.input[symbol.edges[1].0][symbol.edges[1].1] as usize];

                let gear_ratio = gear_0 * gear_1;

                self.gear_ratios.push(gear_ratio);

                log::debug!("Found gear ratio: {} * {} = {}", gear_0, gear_1, gear_ratio);
            }
        }
    }

    pub fn get_sum_of_part_numbers(&self) -> u32 {
        let mut sum: u32 = 0;

        for part_number in &self.part_numbers {
            sum += part_number;
        }

        sum
    }

    pub fn get_sum_of_gear_ratios(&self) -> u32 {
        let mut sum: u32 = 0;

        for gear_ratio in &self.gear_ratios {
            sum += gear_ratio;
        }

        sum
    }
}

pub fn solve(input: std::path::PathBuf, part: Part) -> u32 {
    let file = std::fs::read_to_string(input).unwrap();
    let lines = file.lines();

    let schematic = Schematic::from_lines(lines);

    if part == Part::One {
        return schematic.get_sum_of_part_numbers();
    }

    schematic.get_sum_of_gear_ratios()
}

#[cfg(test)]
mod tests {
    use super::{solve, Part};
    use crate::utils::test::get_input_path;

    #[test]
    fn solve_part_one() {
        assert_eq!(solve(get_input_path(3), Part::One), 532445);
    }

    #[test]
    fn solve_part_two() {
        assert_eq!(solve(get_input_path(3), Part::Two), 79842967);
    }
}
