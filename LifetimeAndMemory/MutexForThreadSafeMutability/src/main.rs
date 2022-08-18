use std::rc::Rc;
use std::sync::{Arc,Mutex};
use std::thread;

/*
    Arc does not protect the variable from concurrent 
    Access, only proctects it from multiple references

    We can put a Mutex around the variable.
    Mutual Exclusion
    Threads are mutually excluded from modifying the
    actual variable until one of them is allowed to do so
*/

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person {
    fn new(name : Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name: name, state: state }
    }

    fn greet(&self) {

        /*
          First we have to lock the mutex
          to prevent other threads from modifying
          the variable while the thread is doing something
          relevant to the variable
        */
        
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        
        
        println!("Hi, {}, I'm {}", self.name, state.as_str());

    }
}

fn mutex_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    //We need to lock the variable before we can use it
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}

fn main() {
    mutex_demo();
}
