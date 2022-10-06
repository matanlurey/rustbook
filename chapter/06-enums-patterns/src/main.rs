fn main() {
    primitive_example();
    concise_example();
    stdlib_option();
    value_in_cents();
}

/// OK: Satisfies the problem, but now we have duplicate data.
#[allow(dead_code)]
#[allow(unused_variables)]
fn primitive_example() {
    // Where structs group together fields, like a Rectangle, enums group together a possible set of values.
    #[derive(Debug)]
    enum IpAddressKind {
        V4,
        V6,
    }

    // Both the address data and the kind of address.
    #[derive(Debug)]
    struct IpAddress {
        kind: IpAddressKind,
        address: String,
    }

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    dbg!(home);

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
    dbg!(loopback);
}

/// Better.
#[allow(dead_code)]
#[allow(unused_variables)]
fn concise_example() {
    #[derive(Debug)]
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress::V4(127, 0, 0, 1);
    dbg!(home);

    let loopback = IpAddress::V6(String::from("::1"));
    dbg!(loopback);
}

/// Showcases the `Option` enum.
#[allow(dead_code)]
#[allow(unused_variables)]
fn stdlib_option() {
    // Nullable values that have a value.
    let some_number = Some(5);
    match some_number {
        None => println!("None"),
        Some(e) => println!("Some <{}>", e),
    }

    // Nullable value that doesn't have a value (i.e. similar to "null").
    let absent_number: Option<i32> = None;
    match absent_number {
        None => println!("None"),
        Some(e) => println!("Some <{}>", e),
    }
}

/// Showcases defining and using enums with pattern matching.
#[allow(dead_code)]
fn value_in_cents() {
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    impl Coin {
        fn value(&self) -> u8 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(_) => 25,
            }
        }
    }

    let coin = Coin::Dime;
    println!("The value of a dime is {}", coin.value());
}
