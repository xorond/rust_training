fn main() {


    // Scalar Types

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition

    let sum = 5 + 10;

    // subtraction

    let difference = 95.5 - 4.3;

    // multiplication

    let product = 4 * 30;

    // division

    let quotient = 56.7 / 32.2;

    // remainder

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';


    // Compound Types

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // arrays

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let first = a[0];
    let seconds = a[1];
}
