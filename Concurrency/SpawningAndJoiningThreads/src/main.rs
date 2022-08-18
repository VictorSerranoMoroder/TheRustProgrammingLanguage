/*
    One of the reasons why Rust focuses so much on memory
    safety has to do with its support for concurrency

*/

use std::thread;
use std::time;

fn main() {


    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(300));
        }   
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    //We wait for the thread to finish
    handle.join();
}
