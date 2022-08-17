use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side:f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side*self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

                        //You can Add multiple traits
fn print_info(shape : impl Shape + Debug) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

//Trait Bound syntax, good when repeating same traits across
//multiple parameters
fn print_info2<T: Shape + Debug>(shape: T){
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn print_info3<T>(shape: T)
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}



fn main() {
    let c = Circle{ radius: 2.0};
    print_info(c);
}
