use colored::Color;
use colored::Colorize;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use rayon::prelude::*;
use regex::Regex;
use std::io;

fn collatz(mut n: BigInt) -> Vec<BigInt> {
    let mut sequence: Vec<BigInt> = vec![n.clone()];

    while n != BigInt::one() {
        if n.clone() % &BigInt::from(2) == BigInt::zero() {
            n /= &BigInt::from(2);
        } else {
            n = &BigInt::from(3) * n.clone() + BigInt::one();
        }
        sequence.push(n.clone());
    }
    sequence
}

fn parse_input(input_value: String) -> Result<BigInt, &'static str> {
    let re = Regex::new(r"(\d+)\^(\d+)(?:-(\d+))?").unwrap();
    if let Some(captures) = re.captures(&input_value) {
        let base = captures[1].parse::<u32>().unwrap();
        let exponent = captures[2].parse::<u32>().unwrap();
        let subtract = captures
            .get(3)
            .map(|m| m.as_str())
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap();
        Ok(BigInt::from(base).pow(exponent) - BigInt::from(subtract))
    } else {
        match input_value.trim().parse::<BigInt>() {
            Ok(value) if value > BigInt::zero() => Ok(value),
            _ => Err("Invalid input. Please enter a valid positive integer > 0"),
        }
    }
}

fn main() {
    println!("Enter an integer for the Collatz sequence (e.g.,27 or 2^199-1 or 2^199):");
    println!();

    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let input_value = match parse_input(input_value) {
        Ok(value) => value,
        Err(err_msg) => {
            println!("{}", err_msg);
            return;
        }
    };
    println!("Parsed input: {}", input_value);
    println!();

    let sequence = collatz(input_value);
    let (max_value, max_index) = sequence
        .par_iter()
        .enumerate()
        .max_by_key(|(_, value)| (*value).clone())
        .map(|(index, _)| (sequence[index].clone(), index))
        .unwrap_or((BigInt::zero(), 0));

    let mut even = BigInt::zero();
    let mut odd = BigInt::zero();

    for num in sequence.iter().skip(1) {
        let color = if num.clone() % &BigInt::from(2) == BigInt::zero() {
            even += BigInt::one();
            Color::White
        } else {
            odd += BigInt::one();
            Color::Yellow
        };
        let formatted_num = if num == &max_value {
            num.to_string().bold().blink().blue()
        } else {
            num.to_string().color(color)
        };
        print!("{} ", formatted_num);
    }

    println!();
    println!();
    println!("stopping time: {}", sequence.len() - 1);
    println!();
    println!("max value:\n\n{}", max_value);
    println!();
    println!("max pos: {}", max_index);
    println!("even: {}", even);
    println!("odd: {}", odd);
    println!();
}
