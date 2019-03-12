use std::collections::HashMap;

fn main() {

    //let mut scores = HashMap::new();

    //scores.insert(String::from("Blue"), 10);
    //scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // type annotation <_, _> is needed here because it's possible to collect()
    // into many different data structures and rust doesn't know which you want
    // unless you specify.
}

fn ownership_stuffs() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,
    // try using them and see what compiler error you get!

    //println!("{}", field_name);
    // compile error: value borrowed after move
}
