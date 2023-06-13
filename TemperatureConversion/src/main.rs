fn main() {
    let fahrenheit: f64 = 180.0;
    let fahrenheit2CelsiusResult = fahrenheit2Celsius(fahrenheit);
    println!("華氏{fahrenheit}為攝氏{fahrenheit2CelsiusResult}！");

    let celsius: f64 = 198.0;
    let celsius2FahrenheitResult = celsius2Fahrenheit(celsius);
    println!("攝氏{celsius}為華氏{celsius2FahrenheitResult}！");
}

fn fahrenheit2Celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius2Fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}