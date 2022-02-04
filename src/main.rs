use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");
    println!("Please input which unit you are converting from. (c/f)");

    let mut unit_in = String::new();

    io::stdin()
        .read_line(&mut unit_in)
        .expect("Failed to read line");

    let unit_in = unit_in.trim();

    let (unit_in, unit_out) = {
        if unit_in == "c" || unit_in == "C" {
            ("Celsius","Fahrenheit")
        } else if unit_in == "f" || unit_in == "F" {
            ("Fahrenheit", "Celsius")
        } else { 
            println!("Invalid argument!");
            println!("Exiting...");
            return;
        }
    };

    println!("Please input the value to convert from {} to {}.", unit_in, unit_out);
    
    let mut value_in = String::new();

    io::stdin()
        .read_line(&mut value_in)
        .expect("Failed to read line");

    let value_in = match value_in.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid argument!");
            println!("Exiting...");
            return;
        }
    };

    let value_out = {
        if unit_in == "Celsius" {
            c_to_f(value_in)
        } else if unit_in == "Fahrenheit" {
            f_to_c(value_in)
        } else {
            println!("unknown problem occured");
            return;
        }
    };
    
    println!("{}° {} corresponds to {}° {}.", value_in, unit_in, value_out, unit_out);
}

fn c_to_f(c: f32) -> f32 {
    (9./5.) * c + 32.
}

fn f_to_c(f: f32) -> f32 {
    (5./9.) * (f - 32.)
}
