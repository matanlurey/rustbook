mod roll {
    // Trait for generating random numbers.
    use rand::Rng;

    pub fn roll_d6() -> u8 {
        rand::thread_rng().gen_range(1..=6)
    }
}

/// Does a thing.
///
/// This intentionally does nothing interesting other than import and call another function.
pub fn do_thing() -> u8 {
    roll::roll_d6()
}
