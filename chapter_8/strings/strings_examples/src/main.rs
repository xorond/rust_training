fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    //let s = "initial contents".to_string();

    // updating a string

    let mut s2 = String::from("foo");
    s2.push_str("bar"); // foobar

    let mut s3 = String::from("lo");
    s3.push('l'); // lol

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s5 is {}", s5);

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; // note s6 has been moved here and can no longer be used

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");

    //let s12 = s9 + "-" + &s10 + "-" + &s11;
    let s12 = format!("{}-{}-{}", s9, s10, s11);

    let len = String::from("Hola").len();

    // Iterating over a string

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

}
