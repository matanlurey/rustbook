# Common Collections[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch08-00-common-collections.html>

To run the program:

```sh
$ cargo run --bin common-collections
   Compiling common-collections v0.1.0 ...
```

The built-in array and tuple types are fixed-size.

The useful data structures in Rust's standard library, called _collections_,
store data on the heap (i.e. the amount of data does not need to be known at
compile-time and can grow or shrink as the program runs).

Will be reviewed:

- A _vector_ (variable number of values next to each other).
- A _string_ (a collection of characters).
- A _hash map_ (associate a value with a particular key).

## Lessons Learned

### Storing Lists of Values with Vectors

A `Vec<T>`, which is similar to a _vector_ in C++, or `ArrayList` in Java:

```rs
// Immutable; no initial value (type explicit).
let v: Vec<i32> = Vec::new();

// Immutable; with initial values (type inferred).
let v = vec![1, 2, 3];

// Mutable; no initial value.
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
```

We can use `[]` or `get` to retrieve values from the vector `v`:

```rs
// Panics if there is no element 2.
//
//         Gets a reference to the value, versus copying it.
//         v      v
let third: &i32 = &v[2];
println!("The third element is {}", third);

// Returns a Result.
let third: Option<&i32> = v.get(2);
match third {
  Some(third) => println!("The third element is {}", third),
  None => println!("There is no third element."),
}
```

The reason you read values from vectors as _references_ (i.e. borrowing) is to
avoid pointing to deallocated memory; the ways vectors work is adding a new
element might require allocating new memory/copying old elements to the new
space, and in the process of doing that the old space might need to removed.

```rs
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

// This will not compile because "first" was possibly derefrenced!
println!("The first element is: {}", first);
```

To iterate over the values of a vector `v`:

```rs
let v = vec![100, 32, 57];
for i in &v {
  println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

To store values that are of a different type (i.e. not _invariant_) use enums:

```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

This is because Rust needs to know what types are in the vector at compile-time
(to know how much memory on the heap will be needed to store each element), so
we must be explicit about what types are allowed in the vector.

> **NOTE**: `trait` will help if the exhaustive set of types are not known.
>
> See Chapter 17.

Like any other `struct`, a vector is freed when it goes out of scope:

```rs
{
  let v = vec![1, 2, 3];
}
```

### Storing UTF-8 Encoded Text with Strings

Recall:

- `&str`, or _string slices_, are string literals stored in the binary.
- `String` is a growable, mutable, owned, UTF-8 encoded string type.
  - `String` is basically a `Vec<u8>` that stores _characters_.

Some interesting methods include _concatenation_:

```rs
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

// Note s1 has been moved here and can no longer be used/.
let s3 = s1 + &s2;
```

Or for multiple strings:

```rs
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// Similar to println!, but returns a String instead.
let s = format!("{}-{}-{}", s1, s2, s3);
```

Interestingly, Rust strings don't support indexing (i.e. `s[0]`):

```rs
// When encoded in UTF-8, the first byte of З is 208 and the second is 151, so
// it would seem that answer should in fact be 208, but 208 is not a valid
// character on its own.
//
// To avoid this class of errors, Rust doesn't compile this code at all!
// (Additionally, indexing is expected to be O(1), but Strings can't guarantee)
let hello = "Здравствуйте";
let answer = &hello[0];
```

However, you can use _slices_:

```rs
let hello = "Здравствуйте";

// Each character is 2-bytes, which means s = "Зд".
let s = &hello[0..4];

// Will cause a runtime panic (similar to indexing into a Vec), as byte '1' is
// not a character boundary.
let s = &hello[0..1];
```

To iterate over strings:

```rs
// For unicode scalar values.
for c in "Зд".chars() {
  // З
  // д
  println!("{}", c);
}

// For bytes.
for b in "Зд".bytes() {
  // 208
  // 151
  // 208
  // 180
  println!("{}", b);
}
```

### Storing Keys with Associated Values in Hash Maps

One way to create a hash map is with `HashMap::new` and `insert`:

```rs
use std::collections::HashMap;

// Like vectors, hash maps are homogeneous (all {keys, values} are same type).
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

To access values, we use `get` or iteration:

```rs
// get the associated value, and handle a missing element.
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
println!("Blue: {}", score);

// iterate over every pair.
for (key, value) in &scores {
  println!("{}: {}", key, value);
}
```

To update a hash map:

```rs
// Will print {"Blue": 25} since 10 was overwritten.
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
println!("{:?}", scores);

// Will still print {"Blue": 25} since it already exists.
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores);
```

Like vectors, we can iterate and update existing values too:

```rs
for (key, *value) in &scores {
  value += 1;
}
```

> **NOTE**: `HashMap` by default uses a slower but DoS-resistant hash function.
>
> See: <https://en.wikipedia.org/wiki/SipHash>.
