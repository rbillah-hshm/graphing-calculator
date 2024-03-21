use crate::algebra_parser;
use std::f32::consts::PI;

type Order = i8;
fn factorial(number: isize) -> isize {
    return (1..=number).into_iter().product();
}
fn gamma(n: f32) -> f32 {
    // Gergő's Formula
    return n.powf(n) * (2.0 * PI * n).sqrt() * (1.0 / ((12.0 * n) + (2.0 / (5.0 * n))) - n).exp();
}
fn power_rule(order: Order, coefficient: f32, exponent: f32, variable: char) {
    let mut buffer = String::new();
    let new_exponent = exponent - order as f32;
    let mut new_coefficient = coefficient;
    if (exponent.floor() == exponent) {
        new_coefficient =
            coefficient * (factorial(exponent as isize) / factorial(order as isize)) as f32;
    } else {
        new_coefficient = coefficient * (gamma(exponent) / factorial(order.into()) as f32);
    }
    buffer.push_str(&coefficient.to_string());
    buffer.push(variable);
    buffer.push_str(&*("^".to_string() + &*new_exponent.to_string()));
}
pub mod derivative {
    use super::*;
    fn compute_expression(expression: &str, order: Order) {}
}
