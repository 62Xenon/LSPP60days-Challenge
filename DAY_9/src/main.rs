mod math;
use std::env;
use std::process;

fn parse_arg(arg: &str) -> Result<f64, String> {
    arg.parse::<f64>()
        .map_err(|_| format!("Invalid number: {}", arg))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <add|sub|mul|div|mode|pow|sqrt|abs|fact> <num1> [num2]",
            args[0]
        );
        process::exit(1);
    }

    let op = &args[1];
    let num1 = parse_arg(&args[2]).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let num2 = if args.len() > 3 {
        parse_arg(&args[3]).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
    } else {
        0.0
    };

    let result = match op.as_str() {
        "add" => Ok(math::add(num1, num2)),
        "sub" => Ok(math::sub(num1, num2)),
        "mul" => Ok(math::mul(num1, num2)),
        "div" => math::div(num1, num2),
        "mode" => math::mode(num1, num2),
        "pow" => Ok(math::pow(num1, num2)),
        "sqrt" => math::sqrt(num1),
        "abs" => Ok(math::abs(num1)),
        "fact" => math::fact(num1),
        _ => Err(format!("Unsupported operation: {}", op)),
    };

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
