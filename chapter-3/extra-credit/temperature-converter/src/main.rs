use std::io;
use std::process;

fn main() {
    let input = read_input();
    let (number, unit) = parse_input(&input);
    print_conversions(number, &unit);
}

fn read_input() -> String {
    let mut input = String::new();
    println!("Enter a temperature and its unit (e.g. '100 c', '212 f', '373.15 k', '671.67 r'):");

    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error: could not read from stdin");
        process::exit(1);
    }

    input
}

fn parse_input(input: &str) -> (f64, String) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() < 2 {
        eprintln!("Error: please provide both a number and a unit (e.g. '100 c')");
        process::exit(1);
    }

    let number: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", parts[0]);
            process::exit(1);
        }
    };

    let unit = parts[1].to_lowercase();

    if !["c", "f", "k", "r"].contains(&unit.as_str()) {
        eprintln!(
            "Error: unknown unit '{}'. Valid units are: c, f, k, r",
            unit
        );
        process::exit(1);
    }

    let below_absolute_zero = match unit.as_str() {
        "k" => number < 0.0,
        "r" => number < 0.0,
        "c" => number < -273.15,
        "f" => number < -459.67,
        _ => unreachable!(),
    };

    if below_absolute_zero {
        eprintln!("Error: temperature is below absolute zero");
        process::exit(1);
    }

    (number, unit)
}

fn print_conversions(number: f64, unit: &str) {
    println!("Fahrenheit : {:.2} f", convert_to_fahrenheit(number, unit));
    println!("Celsius    : {:.2} c", convert_to_celsius(number, unit));
    println!("Kelvin     : {:.2} k", convert_to_kelvin(number, unit));
    println!("Rankine    : {:.2} r", convert_to_rankine(number, unit));
}

// Convert anything to Celsius as the common base
fn to_celsius(number: f64, unit: &str) -> f64 {
    match unit {
        "c" => number,
        "f" => (number - 32.0) * 5.0 / 9.0,
        "k" => number - 273.15,
        "r" => (number - 491.67) * 5.0 / 9.0,
        _ => unreachable!(), // already validated in parse_input
    }
}

fn convert_to_fahrenheit(number: f64, unit: &str) -> f64 {
    to_celsius(number, unit) * 9.0 / 5.0 + 32.0
}

fn convert_to_celsius(number: f64, unit: &str) -> f64 {
    to_celsius(number, unit)
}

fn convert_to_kelvin(number: f64, unit: &str) -> f64 {
    to_celsius(number, unit) + 273.15
}

fn convert_to_rankine(number: f64, unit: &str) -> f64 {
    (to_celsius(number, unit) + 273.15) * 9.0 / 5.0
}

#[cfg(test)]
mod tests;
