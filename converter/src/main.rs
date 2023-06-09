use std::io;

fn main() {
    let mut temp: String = String::new();
    let stdin: io::Stdin = io::stdin();
    println!("Welcome to Fº/Cº/K/R converter!");
    println!("Please enter the temperature you want to convert:");

    stdin
        .read_line(&mut temp)
        .expect("Cannot read temperature😟");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut scale: String = String::new();
    println!("What scale is this temperature in? Enter 'F' for Fahrenheit, 'C' for Celsius, 'K' for Kelvin and 'R' for Rankine:");

    stdin.read_line(&mut scale).expect("Cannot read scale😟");

    let scale: String = scale.trim().to_uppercase();

    match scale.as_str() {
        "F" => {
            println!("Converting to Celsius, Rankine and Kelvin:");
            let celsius: f64 = converter_cel(temp);
            let rankine: f64 = converter_fahr_to_rankine(temp);
            let kelvin: f64 = converter_cel_to_kelvin(celsius);
            print_temp_message(celsius, "Celsius");
            print_temp_message(rankine, "Rankine");
            print_temp_message(kelvin, "Kelvin");
        }
        "C" => {
            println!("Converting to Fahrenheit, Kelvin and Rankine:");
            let fahr: f64 = converter_fahr(temp);
            let rankine: f64 = converter_fahr_to_rankine(fahr);
            let kelvin: f64 = converter_cel_to_kelvin(temp);
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(kelvin, "Kelvin");
            print_temp_message(rankine, "Rankine");
        }
        "R" => {
            println!("Converting to Fahrenheit, Celsius and Kelvin:");
            let fahr: f64 = converter_rankine_to_fahr(temp);
            let celsius: f64 = converter_cel(fahr);
            let kelvin: f64 = converter_cel_to_kelvin(celsius);
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(celsius, "Celsius");
            print_temp_message(kelvin, "Kelvin");
        }
        "K" => {
            println!("Converting to Celsius, Fahrenheit and Rankine:");
            let celsius: f64 = converter_kelvin_to_cel(temp);
            let fahr: f64 = converter_fahr(celsius);
            let rankine: f64 = converter_fahr_to_rankine(fahr);
            print_temp_message(celsius, "Celsius");
            print_temp_message(fahr, "Fahrenheit");
            print_temp_message(rankine, "Rankine");
        }
        _ => {
            println!("Invalid scale entered.");
            return;
        }
    };

    println!("\nPress Enter to exit...");

    let mut _temp: String = String::new();
    io::stdin()
        .read_line(&mut _temp)
        .expect("Failed to read line");
}
fn converter_cel(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}
fn converter_fahr(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}
fn converter_kelvin_to_cel(x: f64) -> f64 {
    x - 273.15
}

fn converter_cel_to_kelvin(x: f64) -> f64 {
    x + 273.15
}
fn converter_fahr_to_rankine(x: f64) -> f64 {
    x + 459.67
}

fn converter_rankine_to_fahr(x: f64) -> f64 {
    x - 459.67
}
fn print_temp_message(temp: f64, scale: &str) {
    let message = match scale {
        "Celsius" => {
            if temp < 0.0 {
                "That's really cold!❄️"
            } else if temp >= 30.0 {
                "That's really hot!🔥"
            } else {
                "That's a moderate temperature."
            }
        }
        "Fahrenheit" => {
            if temp < 32.0 {
                "That's really cold!❄️"
            } else if temp >= 86.0 {
                "That's really hot!🔥"
            } else {
                "That's a moderate temperature."
            }
        }
        "Kelvin" => {
            if temp < 273.15 {
                "That's really cold!❄️"
            } else if temp >= 303.15 {
                "That's really hot!🔥"
            } else {
                "That's a moderate temperature."
            }
        }
        "Rankine" => {
            if temp < 491.67 {
                "That's really cold!❄️"
            } else if temp >= 545.67 {
                "That's really hot!🔥"
            } else {
                "That's a moderate temperature."
            }
        }
        _ => "Invalid scale.",
    };

    println!(
        "The converted temperature is {:.2} degrees {}. {}",
        temp, scale, message
    );
}
