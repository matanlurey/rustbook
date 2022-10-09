#[cfg(test)]
mod tests {
    // Brings all of the non-test paths into scope. Neat!
    use super::*;

    // Indicates that this ia test function, and the test runner should treat this function as a test.
    //
    // We can also have non-test functions to perform common operations, so this annotation is needed.
    #[test]
    fn it_works() {
        let result = 2 + 2;

        // Macro that asserts two values are equal, according to PartialEq.
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn does_not_work() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(4, result, "Number did not add 2, value was {result}");
    }

    #[test]
    #[should_panic(expected = "between 1 and 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn using_result_in_tests() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}

// Code is considered dead_code if it's only used within tests, which is fucking awesome.
// (Another option would be making this struct public)
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
