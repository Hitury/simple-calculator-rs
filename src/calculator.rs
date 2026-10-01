use std::io;

fn validate_equation(args: &Vec<&str>) -> Result<(), String> {
    if args.is_empty() || args.len() % 2 == 0 {
        return Err(String::from(
            "Please use a valid equation format: <number> <operator> <number> ..."
        ))
    }

    for (index, &value) in args.iter().enumerate() {
        if index % 2 == 0 {
            if value.parse::<f64>().is_err() {
                return Err(format!("`{}` is not a valid number.", value))
            }
        }
        else if !matches!(value, "+" | "-" | "/" | "*" | "%") {
            return Err(format!("`{}` is not a valid operator.", value))
        }
    }

    Ok(())
}

fn split_equation(args: &Vec<&str>) -> Result<(), String> {



    Ok(())
}

pub fn calculate() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Please enter a calculation");

    let input = input.trim();
    let _args: Vec<&str> = input.split_whitespace().collect();

    if let Err(message) = validate_equation(&_args) {
        println!("{}", message);
        return;
    }

    let num1: f64 = match _args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error, enter a valid number first");
            return;
        }
    };

    let num2: f64 = match _args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error, enter a valid number first");
            return;
        }
    };
    let op = _args[1];

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "%" => {
            if num2 == 0.0 {
                println!("Cannot divide by 0");
                return;
            }
            num1 / num2
        }
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by 0");
                return;
            }
            num1 / num2
        }
        _ => panic!("Error"),
    };

    println!("Result: {}", result)
}
