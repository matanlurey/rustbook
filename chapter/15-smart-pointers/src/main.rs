use std::{cell::RefCell, ops::Deref, rc::Weak};

fn main() {
    box_smart_pointer();
    cleanup_with_drop();
    immutable_reference_counting();
    mutable_reference_counting();
    prevent_reference_cycles_with_weak();
}

fn box_smart_pointer() {
    // A single value on the heap isn't actually very useful, this is just a simple example.
    let b = Box::new(5);
    println!("b = {b}");

    // How about recurisve types, or a pair of nested pairs (Lisp version of a linked list).
    // (1, (2, (3, Nil)))

    // ERROR: Recursive type has infinite size.
    // vvvvvvvvvvv
    // enum List {
    //    Cons(i32, List),
    //    Nil,
    // }
    //
    // We can use Box to avoid having to know the size at compile-time:
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // What about following a pointer to the value?
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ERROR: No implementation for {integer} == &{integer}
    // assert_eq!(5, y);
    //
    // We can use Box<T> like a reference:
    let x = 5;
    let y = Box::new(x);

    // Just like &x, Box::new(x) can be dereferenced:
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implementing our own "Box"-like type:
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        // Syntax defines an associated type for the Deref trait to use.
        // Associated types are a slightly different way of declaring a generic parameter.
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // Deref coercion, implicitly adds & and * for Deref traits.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // How it would be done without Deref coercion:
    hello(&(*m)[..]);
}

fn cleanup_with_drop() {
    struct CustomSmartPointer {
        data: String,
    }

    // Drop lets you customize what happens when a value is about to go out ouf scope.
    //
    // Typically this can be used to release resoruces such as files or network connections.
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");

    // ERROR: Explicit destructor calls not allowed (to avoid double free).
    // c.drop();
    // However this is:
    std::mem::drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}

fn immutable_reference_counting() {
    // When ownership is unclear (multiple ownership), reference counting can be useful.
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;
    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn mutable_reference_counting() {
    // When ownership is unclear (multiple ownership), and you need to mutate data
    // ... even when there are IMMUTABLE references to it.
    // ... useful for mock objects, or test doubles.
    //
    // This is an unsafe pattern, and can't be checked at compile-time, so it's checked at runtime instead.
    //
    // For example:
    // let x = 5;
    // let y = &mut x;
    //
    // ... would not compile, i.e. with "ERROR: Cannot borrow `x` as mutable".

    trait Messenger {
        // Takes an immutable reference to self.
        fn send(&self, message: &str);
    }

    struct Limiter<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T: Messenger> Limiter<'a, T> {
        fn new(messenger: &'a T, max: usize) -> Limiter<'a, T> {
            Limiter {
                messenger,
                value: 0,
                max,
            }
        }

        fn set(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.7 {
                self.messenger
                    .send("Warning: You've used up over 70% of your quota!");
            }
        }
    }

    // Now let's assume we want to mock Messenger.
    struct MockMessenger {
        // Not used, see ERROR: below on why.
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // ERROR: Cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // Now let's run a test case:
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = Limiter::new(&mock_messenger, 100);

    limit_tracker.set(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

fn prevent_reference_cycles_with_weak() {
    // FIXME
}
