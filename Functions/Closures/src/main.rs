fn say_hello() { println!("Hello!"); }

fn closures() {
    let sh = say_hello;
    sh();

    //Closure, functions that are defined in line
    //Anonymous functions where you can save in a variable
    //or pass as argument to other funcions
    let plus_one = |x:i32| -> i32 { x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            //When two is used in a closure you can't use it
            //anymore while its on the same scope
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    //As we exit the scope we can use two again
    let borrow_two = &mut two;

    // Passing arguments to closures
    //  T: by Value
    //  T&
    // &mut &

    let plus_three = |x:&mut i32| *x += 3;

    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

}

fn main() {
    closures();
}
