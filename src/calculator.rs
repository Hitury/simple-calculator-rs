use std::io;

fn validate_equation(args: &Vec<&str>) -> Result<(), String> {
    if args.is_empty() || args.len() % 2 == 0 {
        return Err(
            String::from("Please use a valid equation format: <number> <operator> <number> ...")
        );
    }

    for (index, &value) in args.iter().enumerate() {
        if index % 2 == 0 {
            if value.parse::<f64>().is_err() {
                return Err(format!("`{}` is not a valid number.", value));
            }
        } else if !matches!(value, "+" | "-" | "/" | "*" | "%") {
            return Err(format!("`{}` is not a valid operator.", value));
        }
    }

    Ok(())
}

fn split_equation(args: &Vec<&str>) -> Result<(Vec<f64>, Vec<char>), String> {
    let (even_pos, odd_pos): (Vec<_>, Vec<_>) = args
        .into_iter()
        .enumerate()
        .partition(|(idx, _)| idx % 2 == 0);

    let numbers: Vec<f64> = even_pos.into_iter()
        .map(|(_, &val)| val.parse::<f64>().unwrap())
        .collect();
    let operators: Vec<char> = odd_pos.into_iter()
        .map(|(_, &val)| val.chars().next().unwrap())
        .collect();

    Ok((numbers, operators))
}

fn evaluate_expression(numbers: &[f64], operators: &[char]) -> f64 {
    // First pass: Handle multiplication, division, and modulo (left to right)
    let mut nums = numbers.to_vec();
    let mut ops = operators.to_vec();
    
    let mut i = 0;
    while i < ops.len() {
        match ops[i] {
            '*' | '/' | '%' => {
                let result = match ops[i] {
                    '*' => nums[i] * nums[i + 1],
                    '/' => nums[i] / nums[i + 1],
                    '%' => nums[i] % nums[i + 1],
                    _ => unreachable!(),
                };
                nums.splice(i..i+2, [result]);
                ops.remove(i);
            },
            _ => i += 1,
        }
    }
    
    // Second pass: Handle addition and subtraction (left to right)
    let mut result = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => result += nums[i + 1],
            '-' => result -= nums[i + 1],
            _ => {} // This shouldn't happen due to validation
        }
    }  
    result
}

pub fn calculate() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Please enter a calculation");

    let input = input.trim();
    let args: Vec<&str> = input.split_whitespace().collect();

    if let Err(message) = validate_equation(&args) {
        println!("{}", message);
        return;
    }

    // Split the equation into numbers and operators
    let (numbers, operators) = match split_equation(&args) {
        Ok(result) => result,
        Err(e) => {
            println!("Error splitting equation: {}", e);
            return;
        }
    };

    // Evaluate the expression with proper operator precedence
    let result = evaluate_expression(&numbers, &operators);
    println!("Result: {}", result)
}
