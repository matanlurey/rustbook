use std::cmp::PartialOrd;

fn main() {
    generic_types_implementing_ordering();
    implementing_traits();
    validating_references_with_lifetimes();
}

fn generic_types_implementing_ordering() {
    /// For an array of items that implement PartialOrd, returns the largest element.
    fn largest<T: PartialOrd>(items: &[T]) -> &T {
        let mut largest = &items[0];

        for item in items {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    /// Derives certain traits (debug, equality, ordering) for this struct.
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Point<T> {
        x: T,
        y: T,
    }

    /// An example of using generics (<T>) on an impl.
    ///
    /// These implementation details will exist on every Point.
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }
    }

    /// An example of using conditional generics (<T: PartialOrd>) on an impl.
    ///
    /// These implementation details will only exist on a Point where T satisfies the PartialOrd trait.
    impl<T: PartialOrd> Point<T> {
        fn cmp(&self) -> &T {
            if self.x >= self.y {
                &self.x
            } else {
                &self.y
            }
        }
    }

    // Numbers support PartialOrd, so they work out of the box.
    let numbers = [6, 1, 8, 2, 3, 14, 9, 12, 4];
    println!(
        "The largest number in {:?}: {:?}",
        &numbers,
        largest(&numbers)
    );

    // Strings too!
    let names = ["Matan", "Sarah", "Bilbo"];
    println!(
        "The largest (last) name in {:?}: {:?}",
        &names,
        largest(&names)
    );

    // Points had to be added manually by deriving #[derive(PartialEq, PartialOrd)].
    let points = [Point::new(1, 1), Point::new(2, 1), Point::new(0, 2)];
    println!(
        "The largest (last) point in {:?}: {:?} (whose largest value is {:?})",
        &points,
        largest(&points),
        // Because a point's values are numbers, and numbers support PartialOrd, then
        // we are allowed to use the ".cmp()" function to pick the larger of the two
        // coordinate values (x or y).
        largest(&points).cmp(),
    );
}

fn implementing_traits() {
    /// An example of creating a trait, sort of an interface.
    trait Summary {
        /// Must be provided by any implementation satisfying Summary.
        fn summarize(&self) -> String;

        /// Can be provided, but otherwise uses this fallback default implementation.
        ///
        /// Worth noting that in Rust, once you've implemented your own method, you can
        /// no longer call the default (i.e. there is no "super.promotion()" or similar).
        fn promotion(&self) -> String {
            String::from("No promotions attached")
        }
    }

    struct Tweet {
        username: String,
        message: String,
    }

    /// Satisfies the Summary trait for Tweet.
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} by {}", self.message, self.username)
        }
    }

    struct Article {
        author: String,
        content: String,
    }

    /// Satisfies the Summary trait for Article.
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{} by {}", self.content, self.author)
        }

        /// Overrides the promotion method.
        fn promotion(&self) -> String {
            String::from("[AD] 0.9% APR on New Car Financing: See https://cars.xyz")
        }
    }

    /// An example of a function that takes trait references instead of values or references directly.
    fn print_summaries(tweet: &impl Summary, article: &impl Summary) {
        println!("Tweet: {} ({})", tweet.summarize(), tweet.promotion());
        println!("Article: {} ({})", article.summarize(), article.promotion());
    }

    let tweet = Tweet {
        username: String::from("matanlurey"),
        message: String::from("Hello World"),
    };

    let article = Article {
        author: String::from("Matan Lurey"),
        content: String::from("Hello World"),
    };

    print_summaries(&tweet, &article);
}

fn validating_references_with_lifetimes() {
    /// With <'a> we get a compile-time error, because the return type is a borrowed value, but the
    /// signature doesn't say whether it's borrowed from x or y. Basically, Rust's compiler needs more
    /// hints about what exactly is being borrowed.
    ///
    /// <'a> is a sort of "generic", and we declare both inputs have thge same lifetime, and the return
    /// value will be one of those lifetimes. This lets the Rust borrow checker/compiler enforce correct
    /// use of this function.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    print!(
        "Is Matan or Mathias Longer?: {}",
        longest("Matan", "Mathias")
    );

    /// Similarily, if we want to hold referenes versus an owned type, we need a lifetime annotation on structs.
    struct Excerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: sentence };
    print!("{}", excerpt.part);
}
