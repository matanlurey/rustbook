use std::{env, process};

mod lib;

fn main() {
    // Collects the command-line arguments into a vector.
    //
    // Note that this function will panic if any argument contains invalid Unicode; there is another function
    // (std::env::args_os), which varies per-platform, that can allow processing something called an "OsString",
    // which is more complex than a "String".
    //
    // Also note the explicit type arguments:
    // That is because collect can create many kinds of collections, and Rust isn't able to infer what kind.
    //        vvvvvvvvvvv
    let args: Vec<String> = env::args().collect();

    // Check if we can continue, as we need at least two arguments.
    assert_eq!(
        args.len(),
        3,
        "Must provide exactly two command-line arguments: <query> <file>, got {} {:?}",
        args.len() - 1,
        &args[1..]
    );

    let query = &args[1];
    assert!(args.len() > 0, "Query must be at least 1 character");

    let path = &args[2];
    assert!(path.len() > 0, "File must be at least 1 character");

    if let Err(e) = crate::lib::run(query, path, env::var("IGNORE_CASE").is_ok()) {
        eprintln!("An error occurred: {e}");
        process::exit(1);
    }
}
