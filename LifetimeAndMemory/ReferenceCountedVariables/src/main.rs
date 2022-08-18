/*
    All these memory garantees from Rust have a price
    You always have to deal with all the idea of Ownership
    And Borrowing

    But there is an alternative, to share a variable around
    without Rust complaining about it.


*/

//Keeps the count of the number of locations in your code
//where the variable is referenced, at 0 the variable can
//be cleaned up. Is a way of keeping track of variables 
//lifetime 
use std::rc::Rc; 

struct Person {
    //name: String
    name: Rc<String>
}

impl Person {
    //fn new(name: String) -> Person {
    fn new(name : Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, {}", self.name);
    }
}

fn rc_demo() {
    //let name = "John".to_string();
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone()); //This adds up to the 
                                            //reference counter
                                            println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
                        
    }
    //as name is now owned by person we can't access it normaly
    println!("{}", name);
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));


}

fn main() {
    rc_demo();  
}
