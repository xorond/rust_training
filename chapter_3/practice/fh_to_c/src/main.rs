use std::io;
use std::io::Write;

fn main() {
    let mut fh = String::new();

    print!("Enter Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fh).expect("failed to read line");
    let fh: f32 = fh.trim().parse().expect("not a float");
    let c = fh_to_c(fh);

    println!("{} F = {} C", fh, c);
}

fn fh_to_c(f: f32) -> f32 {
    // C = (F - 32) * (5/9)
    (f - 32.0) * 5.0/9.0
}
