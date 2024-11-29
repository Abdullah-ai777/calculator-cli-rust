use std::io;

fn main() {
    loop {
        println!("Enter operation (+|-|%|*|/| or clear):");
        let input_traget = input_operator(); 

        match input_traget.as_str() {
            "+" | "-" | "*" | "/" | "%" => {
                operator(&input_traget);
            }
            "clear" => {
                clear_screen(); 
                println!("Screen cleared!");
            }
            "exit" => {
                println!("Exiting the calculator. Goodbye!");
                break;
            }
            _ => println!("Invalid operation!"),
        }
    }
}

// Function to parse numbers
fn input_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number") 
}

// Function to parse operator
fn input_operator() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Function to clear screen
fn clear_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

// Function to handle operations
fn operator(op: &str) {
    println!("Enter num 1:");
    let input_u1 = input_number(); 
    println!("Enter num 2:");
    let input_u2 = input_number(); 
    let result = match op {
        "+" => input_u1 + input_u2,
        "-" => input_u1 - input_u2,
        "*" => input_u1 * input_u2,
        "/" => {
            if input_u2 == 0.0 {
                println!("Error: Cannot divide by 0");
                return;
            }
            input_u1 / input_u2
        }
        "%" => input_u1 % input_u2,
        _ => {
            println!("Invalid operation");
            return;
        }
    };
    println!("Result: {}", result);
}
