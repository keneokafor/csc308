use std::io;

fn main() {
    println!("Enter electricity usage in kWh:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let units: f64 = input.trim().parse().unwrap();

    let rate = if units > 200.0 {
        30.0
    } else if units > 100.0 {
        25.0
    } else {
        20.0
    };

    let bill = units * rate;

    println!("Units used: {units}");
    println!("Rate per unit: ₦{rate}");
    println!("Total bill: ₦{bill}");
}