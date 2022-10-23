# Fearless Concurrency[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch16-00-concurrency.html>

To run the program:

```sh
$ cargo run --bin fearless-concurrency
   Compiling fearless-concurrency v0.1.0 ...
```

## Lessons Learned

Handling concurrent programs safely and efficienctly is a _major_ goal of Rust.

Many languages are dogmatic about handling concurrency problems, i.e. Erland
elegantly handles message-passing concurrency, but has obscure ways to share
state between threads.

Rust, as a lower-level language, tries to provide a variety of tools:

- How to create threads to run multiple pieces of code at the same time.
- _Messsage-passing_ concurrency, where channels send messages between threads.
- _Shared-state_ concurrency, where multiple threads access the same data.
- The `Sync` and `Send` traits, which extends Rust's concurrency guarantees.

### Using Threads to Run Code Simulatenously

To create a new thread, we call `thread::spawn`, and pass it a closure:

```rs
std::thread::spawn(|| {
    for i in 1..10 {
        println!("Number {i} from the spawned thread");
        thread::sleep(Duration::from_millis(1));
    }
});
```

To wait for all threads to finish, we use `join()` handles:

```rs
let handle = std::thread::spawn(|| { /* ... */ });

handle.join().unwrap();
```

#### Using `move` Closures with Threads

Force the `move` closure to take ownership of values rather than infering:

```rs
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
```

#### Using Message Passing to Transfer Data Between Threads

I.e. similar to the _Go_ model, where threads (or actors) communicate by sending
each other messages containing data, which Rust provides in the standard library
called _channels_:

- A _transmitter_
- A _receiver_

```rs
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

//                Blocks the main thread and wait's for a value to be sent.
//
//                Other option is "try_recv", which doesn't block - useful if
//                this thread has other work to do while waiting for messages;
//                we can write a loop checking try_recv from time to time.
//                vvvv
let received = rx.recv().unwrap();
println!("Got: {}", received);
```

To send and receive _multiple_ messages:

1. The sender can send using, for example, a loop:

   ```rs
   for val in vals {
     tx.send(val).unwrap();
   }
   ```

2. The receiver can receive by treating `rx` as an iterator:

   ```rs
   for received in rx {
     println!("Got: {}", received);
   }
   ```

#### Creating Multiple Producers by Cloning the Transmitter

The name `mpsc` stands for _multiple producer, single consumer_. What if we want
to create multiple threads that all send values to the same receiver? We can
_clone_ the transmitter:

```rs
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
```

### Shared-State Concurrency

_Channels_ (message-based concurrency) are similar to _single_ ownership, while
shared memory concurrency is like _multiple ownership_. We can use a _Mutex_
(_mutual exclusion_) to allow only one thread to access data a time, but:

1. You must acquire a lock before using the data.
2. When you are done, you must unlock the data.

```rs
use std::sync::Mutex;
let m = Mutex::new(5);
{
  let mut num = m.lock().unwrap();
  *num = 6;
}
println!("m = {:?}", m);
```

> **TIP**: In the above example, `Mutex` is a _smart pointer_.

#### Sharing a `Mutex<T>` Between Multiple Threads

Rust's compiler will prevent:

- Moving a `Mutex<T>` between threads (not safe to move between threads).
- Moving a `Rc<Mutex<T>>` between threads (not thread-safe).

... but `Arc<T>` _is_ safe to use in concurrent situations (_a_ = _atomic_).

### Extensible Concurrency with the `Sync` and `Send` Traits

So far the concurrency features are part of the Rust standard library, but in
Rust it's possible to write your _own_ concurrency features with the provided
`Sync` and `Send` traits.

#### `Send`: Allow Transfering Ownership Between Threads

Almost every Rust type is `Send` (some exceptions like `Rc<T>`); it means you
can transfer ownership to another thread.

#### `Sync`: Allow Accessing from Multiple Threads

Any Rust type that can referenced from multiple threads is `Sync`; it means you
can reference immutable references, primitive types, and types composed entirely
of `Sync` types.
