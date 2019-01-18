fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // with clone(), the heap data *does* get copied

    println!("s1 = {}, s2 = {}", s1, s2);
}
