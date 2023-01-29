use std::io;

fn main() {
    let mut unit = String::new();

    println!("Enter a unit of temperature: ");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit = unit.trim();


    println!("Enter a temperature in {}: ", unit);
    
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = if unit == "Fahrenheit" {(temperature - 32) * 5 / 9} else {(temperature * 9 / 5) + 32};

    println!("{} degrees {} is {} degrees {}", temperature, {if unit == "Fahrenheit" {"Fahrenheit"} else {"Celsius"}}, result, {if unit == "Celsius" {"Fahrenheit"} else {"Celsius"}});
    
}
