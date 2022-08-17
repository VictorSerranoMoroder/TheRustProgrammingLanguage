fn print_value(x: i32) {
    println!("Value = {}", x);
}

//Paso por parametros
fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y:i32) -> i32 {
    //Without semicolon = return
    x * y
}

fn functions() {
    print_value(33);
    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let p = product(3,5);
    println!("{}",p);
}

fn main() {
    functions();
}
