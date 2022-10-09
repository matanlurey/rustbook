use std::{thread, time::Duration};

fn main() {
    give_away_limited_edition_shirt();
    closure_type_inference();
    capture_references_or_moving_ownership();
    iterators();
}

/// Every so often, our t-shirt company gives away an exclusive, limited-edition shirt as a promotion.
///
/// If a person chose a favorite color, they get that color shirt.
/// Otherwise, they get whatever color the company currently has the most of.
fn give_away_limited_edition_shirt() {
    let store = Inventory {
        shirts: vec![Color::Blue, Color::Red, Color::Blue],
    };

    println!(
        "A user that prefers {:?} gets {:?}",
        Color::Red,
        store.give_away(Some(Color::Red))
    );

    println!(
        "A user without a preference gets {:?}",
        store.give_away(None)
    );

    #[derive(Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    struct Inventory {
        shirts: Vec<Color>,
    }

    impl Inventory {
        fn most_stocked(&self) -> Color {
            // There are many ways to do this, here is one.
            let mut counts = vec![0, 0, 0];

            for color in self.shirts.iter() {
                match color {
                    Color::Red => counts[0] += 1,
                    Color::Green => counts[1] += 1,
                    Color::Blue => counts[2] += 1,
                }
            }

            let most_count = counts.iter().max().unwrap();
            if counts[0] == *most_count {
                return Color::Red;
            } else if counts[1] == *most_count {
                return Color::Green;
            } else {
                return Color::Blue;
            }
        }

        fn give_away(&self, favorite: Option<Color>) -> Color {
            // Neat, use a closure!
            return favorite.unwrap_or_else(|| self.most_stocked());
        }
    }
}

fn closure_type_inference() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    println!("expensive_closure(5): {}", expensive_closure(5));

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;

    add_one_v1(1);
    add_one_v2(2);
    add_one_v3(3);
}

fn capture_references_or_moving_ownership() {
    immutable();
    mutable();
    moving();
    fn_traits();

    fn immutable() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // This captures the scope reference of "list".
        let borrows = || println!("From closure: {:?}", list);
        borrows();

        println!("After calling closure: {:?}", list);
    }

    fn mutable() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // This captures the scope reference of "list".
        let mut borrows_and_mutates = || list.push(4);
        borrows_and_mutates();

        println!("After calling closure: {:?}", list);
    }

    fn moving() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }

    #[allow(dead_code)]
    fn fn_traits() {
        None.unwrap_or_else(|| println!("Won't happen!"));

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
    }
}

fn iterators() {
    let v = vec![1, 2, 3];

    // For-loop, which works on any iterator.
    for v in v.iter() {
        println!("Got: {v}");
    }

    // Or, using the iterator directly.
    let mut i = v.iter();
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), None);

    // Or, consuming the iterator.
    //                     Similar to "collect", sum can't infer the result.
    //                     vvvvvvvvv
    assert_eq!(v.iter().sum::<i32>(), 6);

    // Or, producing an iterator from an iterator.
    let v: Vec<_> = v.iter().map(|x| x + 1).collect();
    assert_eq!(v, vec![2, 3, 4]);

    fn is_greater_than<T: PartialOrd>(items: Vec<T>, item: T) -> Vec<T> {
        items.into_iter().filter(|v| v > &item).collect()
    }

    assert_eq!(is_greater_than(v, 2), vec![3, 4]);
}
