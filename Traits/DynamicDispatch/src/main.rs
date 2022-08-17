
use std::mem;


trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}


fn print_it(z : &dyn Printable) {
    //This function has to look at the type of z and 
    //make a decision of which function to call on
    //runtime, therefore is an expensive
    println!("{}", z.format());
}

struct Circle { radius: f64}
struct Square { side:   f64}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI 
    }
}



fn main() {
    let a = 123;
    let b = "hello".to_string();


    print_it(&a);
    print_it(&b);


    //This implementation can only be done with
    //Dynamic Dispatch
    let shapes:[&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.6},
        &Square{side: 3.5}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has an area of {}", i, shape.area());
    }
}
