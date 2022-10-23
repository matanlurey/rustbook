fn main() {
    creating_a_new_thread_with_spawn();
    waiting_for_all_threads_with_join();
    move_ownership_between_threads();
    pass_messages_between_threads();
    atomic_reference_counting_between_threads();
}

fn creating_a_new_thread_with_spawn() {
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

fn waiting_for_all_threads_with_join() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn move_ownership_between_threads() {
    use std::thread;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn pass_messages_between_threads() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn atomic_reference_counting_between_threads() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
