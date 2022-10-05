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

    // Reference.
    let x = String::from("Hello");
    let l = share_ownership(&x);
    println!("The length of '{}' is {}.", x, l);

    // Compiles without any errors.
    let mut x = String::from("Hello World");
    let w = first_word_without_slices(&x);
    x.clear();
    // ... but does this make any sense to return 5 (it does)?
    println!("The word ends at index: {}", w);

    // Better.
    let x = String::from("Hello World");
    let w = first_word_using_slices(&x);
    // Can't borrow as muitable, since it's borrowed as immutable for the slice.
    // x.clear();
    println!("The first word in {} is: {}", x, w);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn give_ownership(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn share_ownership(some_string: &String) -> usize {
    some_string.len()
}

fn first_word_without_slices(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_using_slices(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
