use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut readme = String::new();
    File::open("README.md")?.read_to_string(&mut readme)?;
    println!("README.md was {} characters", readme.len());
    Ok(())
}
