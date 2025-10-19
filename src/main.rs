use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Type 'C' to convert from Celsius to Fahrenheit, or 'F' to convert from Fahrenheit to Celsius:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().to_uppercase();

    println!("Enter the temperature to convert:");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read line");
    let temp: f64 = temp_input.trim().parse().expect("Please type a number!");

    if choice == "C" {
        let result = (temp * 9.0 / 5.0) + 32.0;
        println!("{temp}°C = {result:.2}°F");
    } else if choice == "F" {
        let result = (temp - 32.0) * 5.0 / 9.0;
        println!("{temp}°F = {result:.2}°C");
    } else {
        println!("Invalid choice. Please type 'C' or 'F'.");
    }
}