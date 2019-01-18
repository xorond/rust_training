use std::io;

fn main() {
    println!("Fibonacci(n) = ");

    let mut step = String::new();

    io::stdin().read_line(&mut step).expect("failed to read line");

    let step: u32 = match step.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("expected an integer number");
            return
        }
    };

    println!("{}th Fibonacci: {}", step, fibonacci(step));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
