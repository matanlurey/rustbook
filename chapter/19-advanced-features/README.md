# Advanced Features[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch19-00-advanced-features.html>

> **Note**: There is no program included in this chapter, as it's notes only.

## Unsafe Rust

All of the code discussed so far has Rust's memory safety guarantees enforced
at compile-time (well, `RefCell<T>` and friends are enforced at runtime, but
good enough).

However, Rust has a _second language_ called _unsafe Rust_, because by nature
static analysis must be conservative, and it's better to reject some valid
programs than accept some invalid programs.

> **NOTE**: Another reason Rust has an unsafe alter ego is ... reality.
>
> Computer hardware is inherently unsafe, and if Rust didn't let you do unsafe
> operations _at all_ then you couldn't do certain tasks (i.e. low-level
> systems programming).

### Unsafe Superwpoers

To switch to unsafe Rust, we use the `unsafe` keyword, which then allows:

- Dereferencing a raw pointer
- Calling an unsafe function or method
- Accessing or modifying a mutable static variable
- Implementing an unsafe trait
- Accessing fields of `union`s

... otherwise, it is identical to other Rust code.

### Derefrencing a Raw Pointer

In [Understanding Ownership](../04-understanding-ownership/README.md#dangling-references)
it was mentioned that the compiler ensures references are always valid. However,
unsafe Rust has two new types called _raw pointers_, that are similat to
references:

- `*const T` (immutable raw pointer)
- `*mut T` (mutable raw pointer)

Compared to references and smart pointers, raw pointers:

- Ignore the borrowing rules by being able to point to the same location
- Aren't guarnateed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup

So, by opting out of these guarnatees (i.e. giving up guaaranteed safety) you
can gain greater performance and/or the ability to interface with another
language or hardware where Rust's guarantees don't apply:

```rs
let mut num = 5;

// Note there is no "unsafe" block, we can create, but not use, raw pointers.
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// Derefencing in an "unsfae" block:
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

// Also possible:
let address = 0x012345usize;
let r = address as *const i32;
```

### Calling an Unsafe Function or Method

For example, implementing the `split_at_mut` function in the standard library:

```rs
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
    }
}
```

### Using `extern` Functions to Call External Code

```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Or, allowing to call Rust functions from other languages:

```rs
// Note, no "unsafe" required.
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just aclled a Rust function from C!");
}
```

### Accessing or Modifying a Mutable Static Variable

```rs
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

Unlike the rest of the "advanced" features, _associated tyhpes_ are not _rare_;
one example of a trait with an associated type is the `Iterator` trait:

```rs
pub trait Iterator {
    // Placeholder.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

For example:

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

Why not use generics, i.e. `pub trait Iterator<T>`? The difference is we
must annotate the types in _each_ implementation, because we can also implement
`Iterator<String> for Counter` (or any other type) - i.e. we can end up with
multiple implementations of `Iterator` for `Counter`.

By using associated types (`type Item`), we don't need to annotate types because
we can't implement a trait on a type multiple times - i.e we guarantee a single
`Iterator for Counter`.

### Default Generic Type Parameters and Operator Overloading

When we use generic typ eparameters, we can specify a default concrete type:

```rs
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

These are helpful for:

- Extending a type without breaking existing code
- Allowing customization in specific cases most users won't need

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

```rs
fn main() {
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

```rs
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

### Using the Newtype Pattern to Implement External Traits on External Types

```rs
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

## Advanced Types

### Using the Newtype Pattern for Type Safety and Abstraction

THe newtype pattern is useful for statically enforicng that values are never
confused and indicating the units of a value, for example the `Millimeters` and
`Meters` structs wrapping `u32`, or `HashMap<i32, String>` becoming `People`.

### Creating Type Synonyms with Type Aliases

```rs
// No extra safety, but can reduce reptition of long types.
type Kilometers = i32;

// Better example
type Thunk = Box<dyn Fn() + Send + 'static>;
```

### The Never Type that Never Returns

```rs
fn bar() -> ! {
    // --snip--
}
```

### Dynamically Sized Types and the Sized Trait

Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type. So, DSTs, or _dynamically sizd types_
or _unsized types_ let us write code using values who size we can only know at
runtime.

For example, `str` (**not** `&str`), is a DST:

```rs
// Does not work; Rust needs to know how much memory to allocate!
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

But, this works:

```rs
// &str is two values: the address of the str and its length.
let s1: &str = "Hello there!";
let s2: &str = "How's it going?";
```

> **TIP**: The golden rule of dynamically sized types is that we must always put
> values of dynamically sized types behind a pointer of some kind.

To work with DSTs, Rust provides the `Sized` trait:

```rs
// Rust implicitly adds a bound on Sized to every generic function:
fn generic<T /*: Sized*/>(t: T) {
    // --snip--
}
```

By default, generic functions only work on types with a known size, but:

```rs
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

... `?Sized` means "`T` may or may not be `Sized`".

## Advanced Functions and Closures

### Function Pointers

The `fn` type is called a _function pointer_:

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

### Returning Closures

```rs
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## Macros

The term _macro_ refers to a family of features in Rust: _declarative_ macros
with `macro_rules!` and three kinds of _procedural_ macros:

- Custom `#[derive]` macros that specify code added with the `derive` atribute
  used on structs and enums.
- Attribute-like macros that define custom attributes usable on any item.
- Function-like macros that look like function calls but operate on the tokens
  specified as their argument.

### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code.

A function signature must declare the number and type of parameters the function
has. Macros, on the other hand, can take a variable number of parameters, i.e.
with `println!`.

### Declarative Macros with `macro_rules!` for General Metaprogramming

A slightly simplifeid form of the `vec!` macro:

```rs
// Macro should be made available whenever the crate is brought in scope.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

In the above case, `vec![1,2, 3]` is replaced by:

```rs
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

### Procedural Macros for Generating Code from Attributes

When creating procedural macros, the definitions must reside in their own crate
with a special crate type:

```rs
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}
```

See also: <https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes>
