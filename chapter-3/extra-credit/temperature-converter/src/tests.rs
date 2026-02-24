use super::*;

// Helper to compare floats with a small tolerance, since f64 arithmetic
// isn't always perfectly precise
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

// --- to_celsius ---

#[test]
fn test_celsius_to_celsius() {
    assert!(approx_eq(to_celsius(100.0, "c"), 100.0));
}

#[test]
fn test_fahrenheit_to_celsius() {
    assert!(approx_eq(to_celsius(212.0, "f"), 100.0));
    assert!(approx_eq(to_celsius(32.0, "f"), 0.0));
}

#[test]
fn test_kelvin_to_celsius() {
    assert!(approx_eq(to_celsius(373.15, "k"), 100.0));
    assert!(approx_eq(to_celsius(273.15, "k"), 0.0));
}

#[test]
fn test_rankine_to_celsius() {
    assert!(approx_eq(to_celsius(671.67, "r"), 100.0));
    assert!(approx_eq(to_celsius(491.67, "r"), 0.0));
}

// --- convert_to_fahrenheit ---

#[test]
fn test_celsius_to_fahrenheit() {
    assert!(approx_eq(convert_to_fahrenheit(100.0, "c"), 212.0));
    assert!(approx_eq(convert_to_fahrenheit(0.0, "c"), 32.0));
}

#[test]
fn test_kelvin_to_fahrenheit() {
    assert!(approx_eq(convert_to_fahrenheit(373.15, "k"), 212.0));
}

// --- convert_to_kelvin ---

#[test]
fn test_celsius_to_kelvin() {
    assert!(approx_eq(convert_to_kelvin(100.0, "c"), 373.15));
    assert!(approx_eq(convert_to_kelvin(0.0, "c"), 273.15));
}

#[test]
fn test_fahrenheit_to_kelvin() {
    assert!(approx_eq(convert_to_kelvin(212.0, "f"), 373.15));
}

// --- convert_to_rankine ---

#[test]
fn test_celsius_to_rankine() {
    assert!(approx_eq(convert_to_rankine(100.0, "c"), 671.67));
    assert!(approx_eq(convert_to_rankine(0.0, "c"), 491.67));
}

// --- parse_input ---

#[test]
fn test_parse_input_valid() {
    let (number, unit) = parse_input("100 c");
    assert!(approx_eq(number, 100.0));
    assert_eq!(unit, "c");
}

#[test]
fn test_parse_input_uppercase_unit() {
    // units should be lowercased automatically
    let (number, unit) = parse_input("100 C");
    assert!(approx_eq(number, 100.0));
    assert_eq!(unit, "c");
}

#[test]
fn test_parse_input_absolute_zero() {
    // exact absolute zero should be allowed
    let (number, unit) = parse_input("0 k");
    assert!(approx_eq(number, 0.0));
    assert_eq!(unit, "k");
}
