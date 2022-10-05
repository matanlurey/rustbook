fn main() {
    /// Example of a struct with three (named) fields: active, email, sign_in_count.
    struct User {
        active: bool,
        email: String,
        sign_in_count: u64,
    }

    // To get values, use dot notation.
    fn print_user(user: &User) {
        println!(
            "User: {} (active = {}, sign_in_count = {})",
            user.email, user.active, user.sign_in_count
        );
    }

    // Creates an immutable struct.
    let user = User {
        active: true,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print_user(&user);

    // Or, a mutable struct (all fields are mutable, or none are mutable).
    let mut user = user;
    user.sign_in_count += 1;
    print_user(&user);

    // There is also a convenient shorthand.
    let active = false;
    let email = String::from("sundar@google.com");
    let sign_in_count = 1001;
    let user = User {
        active,
        email,
        sign_in_count,
    };
    print_user(&user);

    // Or, use ..
    let user = User {
        sign_in_count: sign_in_count + 1,
        ..user
    };
    print_user(&user);

    // There are also tuple structs:
    struct Color(i32, i32, i32);
    let black = Color(0xFF, 0xCA, 0x0C);
    println!("0x{:0>2X}{:0>2X}{:0>2X}", black.0, black.1, black.2);

    // But what about storing references to data owned by someone else?
    //
    // ERROR: Missing lifetime specifier
    // struct User {
    //    active: &bool,
    // }
    //
    // Not covered until chapter 10!

    calculates_area_of_a_rectangle();
}

/// A sample program that uses structs!
fn calculates_area_of_a_rectangle() {
    struct Rectangle1 {
        width: u32,
        height: u32,
    }

    let rect = Rectangle1 {
        width: 30,
        height: 50,
    };

    // Neat, a C-like function :)
    fn area(rectangle: &Rectangle1) -> u32 {
        rectangle.width * rectangle.height
    }
    println!("The area of the rectangle is {} square pixels", area(&rect));

    // Add useful functionality with Derived Traits
    //
    // ERROR: `Rectangle1` doesn't implement `std::fmt::Display`.
    // println!("Rectangle is {}", rect)
    //
    // ERROR: `Rectangle1` doesn't implement `Debug`
    // println!("Rectangle is {:?}", rect)
    //                         ^^ Output format called "Debug", useful for developers.

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle2 {
        width: u32,
        height: u32,
    }
    let rect = Rectangle2 {
        width: 30,
        height: 50,
    };
    //                      vvv A bit easier to read large structs by adding newlines/whitespace.
    println!("Rectangle is {:#?}", rect);

    // We can also print to stderr.
    // dbg!(&rect);

    /// Adds associated (statically dispatched) methods to Rectangle2.
    impl Rectangle2 {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    println!("Rectangle's area is {}", rect.area());
    let rect = Rectangle2::square(10);
    println!("Rectangle {:?}'s area is {}", rect, rect.area());
}
