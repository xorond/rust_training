fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // &String won't compile
    let s = String::from("hello");

    s //&s won't compile because of dangling reference
}
