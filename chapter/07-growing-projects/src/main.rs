/// Looks for a file named "lib.rs" and will insert its contents under the scope "lib".
mod lib;

fn main() {
    let result = lib::do_thing();
    println!("Rolled a D6: {result}");
}
