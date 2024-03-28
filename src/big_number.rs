use num::{traits::real::Real, Float};
use std::ops::{self, BitAndAssign};
const HAVEN_ABBREVIATIONS: [Option<&str>; 9] = [
    None,
    Some("K"),
    Some("M"),
    Some("B"),
    Some("T"),
    Some("QD"),
    Some("QN"),
    Some("SX"),
    Some("SP"),
];
fn bool_from_number(number: i32) -> bool {
    match number {
        -1 => false,
        0 => false,
        _ => true,
    }
}
fn reverse_number(a: i32, b: i32, c: i32) -> i32 {
    *(a..=b)
        .rev()
        .collect::<Vec<i32>>()
        .get((c - a) as usize)
        .unwrap()
}
fn cyclic_wrap(a: i32, b: i32, max: i32) -> f32 {
    (10 as f32).powf((((a + b) % max) - a) as f32)
}
#[derive(Clone)]
pub enum Format {
    Haven(String),
    Scientific(String),
}
#[derive(Clone)]
pub struct BigNumber {
    serialized: Format,
    base: f32,
    exponent: i32,
}
macro_rules! handle_analysis_errors {
    ($condition:expr, $error:expr) => {
        if ($condition) {
            match $error.err().unwrap() {
                AnalysisErrors::InvalidPrefix => {}
                AnalysisErrors::InvalidSuffix => {}
                AnalysisErrors::InvalidExponent => {}
            }
            return;
        }
    };
}
impl BigNumber {
    fn new(serialized: Format) {}
    fn new_d(deserialized: f32) -> BigNumber {
        let mut temp = BigNumber {
            serialized: Format::Haven(("1").to_string()),
            base: 1.0,
            exponent: 0,
        };
        temp.increase_power(deserialized.log10().floor() as i32);
        match temp.serialized {
            Format::Haven(x) => {
                let current_multiplier = Haven::get_multiplier(x);
                Haven::set_multiplier(&mut temp);
            }
            Format::Scientific(x) => {
                Scientific::set_multiplier(&mut temp);
            }
        }
        temp
    }
    pub fn get_value(&self) -> String {
        match self.serialized.clone() {
            Format::Haven(x) => x,
            Format::Scientific(x) => x,
        }
    }
    pub fn increase_power(&mut self, increment: i32) {
        let exponent = match self.serialized.clone() {
            Format::Haven(x) => Haven::get_exponent(x),
            Format::Scientific(x) => Scientific::get_exponent(x),
        };
        handle_analysis_errors!(exponent.is_err(), exponent);
        let new_power = exponent.ok().unwrap() + increment;
        if (new_power > (HAVEN_ABBREVIATIONS.len() * 3) as i32) {
            let multiplier = Scientific::get_multiplier(self.get_value(), increment);
            handle_analysis_errors!(multiplier.is_err(), multiplier);
            self.base = multiplier.ok().unwrap() as f32;
            self.exponent = new_power;
            self.serialized =
                Format::Scientific(Scientific::create(multiplier.ok().unwrap(), new_power));
        } else {
            let multiplier = Haven::get_multiplier(self.get_value(), increment);
            handle_analysis_errors!(multiplier.is_err(), multiplier);
            self.serialized = Format::Haven(Haven::create(multiplier.ok().unwrap(), new_power));
        }
    }
    pub fn decrease_power(&mut self, increment: i32) {
        self.increase_power(-increment)
    }
}
impl ops::Mul<f32> for BigNumber {
    type Output = BigNumber;
    fn mul(self, rhs: BigNumber) -> Self::Output {}
}
enum AnalysisErrors {
    // Haven
    InvalidPrefix,
    InvalidSuffix,
    //
    InvalidExponent,
}
trait NumberMethods {
    fn get_exponent(x: String) -> Result<i32, AnalysisErrors>;
    fn get_multiplier(x: String, exponent: i32) -> Result<i32, AnalysisErrors>;
    fn set_multiplier(x: &mut BigNumber, base: i32);
    fn create(a: i32, b: i32) -> String;
}
struct Haven;
struct Scientific;
impl NumberMethods for Haven {
    fn get_exponent(x: String) -> Result<i32, AnalysisErrors> {
        let mut abbreviation = String::new();
        let rest = x
            .chars()
            .rev()
            .take_while(|char| {
                let is_alphabetic = char.is_ascii_alphabetic();
                if (is_alphabetic) {
                    abbreviation.push(*char);
                }
                is_alphabetic
            })
            .collect::<String>();
        if (abbreviation.is_empty()) {
            let num = x.parse::<i32>();
            let unwrapped = match num {
                Ok(n) => n,
                Err(error) => -1,
            };
            if (!bool_from_number(unwrapped)) {
                return Err(AnalysisErrors::InvalidPrefix);
            }
            return Ok(((unwrapped as f32).log10().floor() + 1.0) as i32);
        } else {
            let mut position = None;
            for (index, suffix) in HAVEN_ABBREVIATIONS.iter().enumerate() {
                if (*suffix.unwrap() == abbreviation) {
                    position = Some(index);
                    break;
                }
            }
            if (position.is_some()) {
                let parsed_rest = rest.parse::<f32>();
                if (parsed_rest.is_err()) {
                    return Err(AnalysisErrors::InvalidPrefix);
                }
                return Ok(((position.unwrap() as i32 * 3 as i32)
                    - reverse_number(1, 3, parsed_rest.ok().unwrap().log10().floor() as i32))
                    + 1);
            } else {
                return Err(AnalysisErrors::InvalidSuffix);
            }
        }
    }
    fn get_multiplier(x: String, exponent: i32) -> Result<i32, AnalysisErrors> {
        match x
            .chars()
            .take_while(|char| char.is_ascii_digit() || *char == '.')
            .collect::<String>()
            .parse::<f32>()
        {
            Ok(number) => Ok((number
                * cyclic_wrap(((number as f32).log10().floor() as i32 + 1), exponent, 3))
                as f32),
            Err(error) => Err(AnalysisErrors::InvalidPrefix),
        }
    }
    fn set_multiplier(x: &mut BigNumber, base: i32) {}
    fn create(a: i32, b: i32) -> String {
        let mut serialized = String::new();
        let abbreviation = HAVEN_ABBREVIATIONS[(b as f32 / 3.0).floor() as usize];
        serialized.push_str(a.to_string().as_str());
        match abbreviation {
            Some(x) => {
                serialized.push_str(abbreviation.unwrap());
            }
            _ => {}
        }
        serialized
    }
}
impl NumberMethods for Scientific {
    fn get_exponent(x: String) -> Result<i32, AnalysisErrors> {
        let exponent = x
            .chars()
            .rev()
            .take_while(|char| char.is_ascii_digit())
            .collect::<String>();
        let parsed_exponent = exponent.parse::<i32>();
        match parsed_exponent {
            Ok(number) => Ok(number),
            Err(error) => Err(AnalysisErrors::InvalidExponent),
        }
    }
    fn get_multiplier(x: String, exponent: i32) -> Result<i32, AnalysisErrors> {
        Haven::get_multiplier(x, exponent)
    }
    fn set_multiplier(x: &mut Self, base: i32) {}
    fn create(a: i32, b: i32) -> String {
        let mut serialized = String::new();
        serialized.push_str(a.to_string().as_str());
        serialized.push_str("x10^");
        serialized.push_str(b.to_string().as_str());
        serialized
    }
}
pub struct BigVec2 {
    x: BigNumber,
    y: BigNumber,
}
