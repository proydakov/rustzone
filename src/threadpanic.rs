use std::env;
use std::thread;
use std::time::Duration;

struct StackObject(&'static str);

impl Drop for StackObject {
    fn drop(&mut self) {
        println!("Drop '{}'", self.0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = if args.len() > 1 { args[1].parse::<i32>().unwrap() } else { 0 };
    println!("{}", data);

    thread::spawn(move || {
        let _object = StackObject("I'm stack data");
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("DIV: {}", 15 / data);
        panic!("Big problem !!!");
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1000));
    }
}
