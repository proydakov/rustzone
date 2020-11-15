use std::thread;

fn main() {
    let _ = thread::Builder::new().name("child1".to_string()).spawn(move || {
        let i = 1;
        loop {
            println!("hi number {} from the spawned thread!", i);
        }
    });
    
    let _ = thread::Builder::new().name("child2".to_string()).spawn(move || {
        let i = 2;
        loop {
            println!("hi number {} from the spawned thread!", i);
        }
    }).unwrap();

    let _ = thread::Builder::new().name("child3".to_string()).spawn(move || {
        let i = 3;
        loop {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    let i = 0;
    loop {
        println!("hi number {} from the main thread!", i);
    }
}
