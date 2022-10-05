fn main() {
    strings();
}

fn strings() {
    // Immutable hardcoded string literal.
    let s: &str = "Hello";
    println!("{s}");

    // Immutable string object (allocated on the heap).
    let s: String = String::from(s);
    println!("{s}");

    // Mutable string object (also allocated on the heap).
    let mut s: String = String::from(s);
    s.push_str(" World!");
    println!("{s}");

    // Heap clone.
    let x = String::from("Hello");
    let y = x.clone();
    println!("{} World", y);

    // Move.
    let x = String::from("Hello");
    take_ownership(x);
    // -- Cannot use "x" at this point.

    // Move.
    let x = String::from("Hello");
    let (x, l) = give_ownership(x);
    println!("The length of '{}' is {}.", x, l);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn give_ownership(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
