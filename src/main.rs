use std::io;
use std::process;

// since String is a growable buffer, it can't be constant. We need type &str in here. Wonder what all this means.
const CELSIUS_TO_FAHRENHEIT : &str = "a";
const FAHRENHEIT_TO_CELSIUS : &str = "b";

fn main() {
    let mut temp_converter: String;
    loop {
        println!("Which do you want to convert?");
        println!("({}) celsius to fahrenheit", CELSIUS_TO_FAHRENHEIT);
        println!("({}) fahrenheit to celsius", FAHRENHEIT_TO_CELSIUS);
        temp_converter = get_user_line();
        if temp_converter == CELSIUS_TO_FAHRENHEIT || temp_converter == FAHRENHEIT_TO_CELSIUS {
            break;
        }
    }
    
    println!("Enter temperature to convert:");
    let temperature = get_user_line();
    
    let temperature : f64 = match temperature.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    
    let original_scale;
    let new_scale;
    let new_temperature;

    if temp_converter == CELSIUS_TO_FAHRENHEIT {
        new_temperature = celsius_to_fahrenheit(temperature);
        original_scale = "c".to_string();
        new_scale = "f".to_string();
    } else if temp_converter == FAHRENHEIT_TO_CELSIUS.to_string() {
        new_temperature = fahrenheit_to_celsius(temperature);
        original_scale = "f".to_string();
        new_scale = "c".to_string();
    } else {
        println!("PANIC TIME");
        process::exit(1);
    }
    println!("Converted {}{} to {}{}", temperature, original_scale, new_temperature, new_scale);
}

fn get_user_line() -> String {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read answer");
    // the read value has a \n in the end of it, trim that stuff out!
    // we also need to coerce it into a string with String::from. Tricky stuff!
    value = String::from(value.trim());
    return value;
}

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    (degrees * 9.0/5.0) + 32.0
}