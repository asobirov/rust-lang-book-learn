use std::io;

fn main() {
    let mut scale = String::new();

    loop {
        println!("Choose a temperature scale [C/F]:");

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        scale = match scale.trim().to_lowercase().as_ref() {
            "c" => "c".to_string(),
            "f" => "f".to_string(),
            _ => continue,
        };

        break;
    }

    let temp: f32;

    loop {
        println!("Enter a temperature: ");
        let mut temp_str = String::new();
        io::stdin()
            .read_line(&mut temp_str)
            .expect("Failed to read line");

        temp = match temp_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    if scale == "c" {
        println!(
            "{} degrees Celsius is {} degrees Fahrenheit",
            temp,
            c_to_f(temp)
        );
    } else {
        println!(
            "{} degrees Fahrenheit is {} degrees Celsius",
            temp,
            f_to_c(temp)
        );
    }
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn c_to_f(temp: f32) -> f32 {
    temp * 9.0 / 5.0 + 32.0
}
