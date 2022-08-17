struct Point {
    x:f32,
    y:f32
}

struct Line {
    start: Point,
    end:   Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0};

    let p2 = Point {x: 4.0, y: 2.0};
    let myline = Line {start: p, end: p2};
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("sp = {:?}", sp);
    println!{"{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1};

    //destructuring
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4,7);
    //Tupple of tupples....
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    
    //Mutiple type Tuple
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    //Single element Tuple
    let meaning = (42,);
    println!("{:?}", meaning);
}

fn main() {
    tuples();
}
