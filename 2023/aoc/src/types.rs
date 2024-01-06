use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("Unsupported part: {}, must be 1-2", s)),
        }
    }
}
