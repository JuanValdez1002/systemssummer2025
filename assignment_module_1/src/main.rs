const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut temp_f = FREEZING_POINT_F;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.1}°F is {:.1}°C", temp_f, temp_c);

    for i in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F is {:.1}°C", temp_f, temp_c);
    }
}