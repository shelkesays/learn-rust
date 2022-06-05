use std::io;

fn fahrenheit(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}

fn celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("Please enter the temprature unit: ");
    let mut unit = String::new();
    io::stdin().read_line(&mut unit).expect("Failed to read the unit.");
    
    unit = unit.trim().to_string();
    println!("Enter the temprature in {}: ", unit);

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read the tempratue.");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let mut new_temp = 0.0;
    let mut new_unit = String::new();

    if unit.eq("celsius") {
        new_temp = fahrenheit(temp);
        new_unit = "fahrenheit".to_string();
    } else {
        new_temp = celsius(temp);
        new_unit = "celsius".to_string();
    }

    println!("{} degree {} = {} degree {}.", temp, unit, new_temp, new_unit);
}
