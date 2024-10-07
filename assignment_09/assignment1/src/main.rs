fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    const FREEZING_POINT: f64 = 32.0;
    let mut fahrenheit_temp: f64 = 34.0;
    let converted_celcius: f64 = fahrenheit_to_celcius(fahrenheit_temp);
    println!("Fahrenheit to celcius: {}", converted_celcius);
    println!("The next 5 temperatures in range: ");
    for _ in 1..=5 {
        fahrenheit_temp += 1.0;
        println!("{}", fahrenheit_to_celcius(fahrenheit_temp));
    }
}
