fn main() {
    // Review of concepts.
    variables_and_mutability();
    constants();
    shadowing();
    floating_point_types();
    numeric_operations();
    explicit_integer_overflow();
    tuples();
    arrays();
    control_flow();

    // Exercises.
    // See: https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary.
    convert_temperatures();
    generate_finobacci();
    twelve_days_of_christmas();
}

fn variables_and_mutability() {
    // if not made "mut", we'd receive "cannot assign assign twice ..." error.
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    // compute this value at compile-time
    // see <https://doc.rust-lang.org/reference/const_eval.html>
    // constants cannot use type inference, so we must put the result (u32).
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    // you can declare a new variable with the same name as a previous one
    let x = 5;
    let x = x + 1;

    // you can also create temporary explicit scopes
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // which retains the previous scope
    println!("The value of x in the outer scope is: {x}");

    // it's also useful for re-using a good name
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces in the text: {spaces}");
}

fn floating_point_types() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}, y: {y}");
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let floored = 2 / 3;
    println!("2 / 3 = {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");
}

fn explicit_integer_overflow() {
    let x: u8 = 255;

    // allows wrapping.
    let y = x.wrapping_add(1);
    println!("(255 as u8).wrapping_add(1) = {y}");

    // returns none if overflow.
    let y = x.checked_add(1);
    let y = match y {
        Some(y) => y.to_string(),
        _ => "None".to_string(),
    };
    println!("(255 as u8).checked_add(1) = {y}");

    // returns both the value and an overflow boolean.
    let (y, o) = x.overflowing_add(1);
    println!("(255 as u8).overflowing_add(1) = {y}, {o}");

    // saturates at the maximum value.
    let y = x.saturating_add(1);
    println!("(255 as u8).saturating_add(1) = {y}");
}

fn tuples() {
    let t = (500, 64, 1);
    let (x, y, z) = t;
    println!("The value of t is {x}, {y}, {z}");
}

fn arrays() {
    let a = [1, 2, 3];
    let [x, y, z] = a;
    println!("The value of a is {x}, {y}, {z}");
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("3 < 5");
    } else {
        panic!("3 is most definitely not >= 5");
    }

    let number = if number < 5 { 5 } else { 6 };
    println!("if 5 < 5 {{ 5 }} else {{ 6 }} = {number}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
}

/// Converts between Farenheit and Celsius.
fn convert_temperatures() {
    // While this could be relatively fancy (using sum types), keep it simple.
    let farenheit = [-459.67, -40.0, 0.0, 32.0, 98.6, 212.0];
    let celsius = farenheit.map(|f| (f - 32.0) / 1.8);
    println!("Farenheit: {:?}", farenheit);
    println!("Celsius:   {:?}", celsius);
}

/// Generates the nth Fibonacci number.
fn generate_finobacci() {
    fn fib(n: u32) -> u32 {
        if n < 2 {
            n
        } else {
            fib(n - 2) + fib(n - 1)
        }
    }

    println!("fib(5)  = {:?}", fib(5));
    println!("fib(13) = {:?}", fib(13));
}

/// Print the lyrics to the Christmas carol "The Twelve Days of Christmas".
fn twelve_days_of_christmas() {
    let numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for verse in 0..verses.len() {
        println!("[Verse {:}]", verse + 1);
        println!(
            "On the {:} day of Christmas, my true love sent to me",
            numbers[verse]
        );
        for line in 0..verse + 1 {
            println!("{:}", verses[verse - line]);
        }
    }
}
