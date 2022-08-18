//Rc is only limited to a single Thread

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name : Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, {}", self.name);
    }
}

fn arc_demo() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    //If you want to pass objects within threads the parts
    //you are passing around have the safe trait and the
    //Rc class does not.

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);
    t.join().unwrap();
}

fn main() {
    arc_demo();
}
