use std::vec;

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    //usize isize
    //idx is a signed variable and also
    //is running on a 64bit machine 
    
    //let idx:i32 = 0; CANT WORK
    let idx:usize = 0;

    println!("a[0] = {}", a[idx]);

    //let idx:usize = 111; THIS CRASHES THE PROGRAM
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);  

    //  Option (Safe code!)
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No element")
    }

    for x in &a { println!("{}",x);}

    //Some(44)
    let last_element = a.pop();
    println!("Last Element is {:?}, a = {:?}", last_element, a);

    //Get values from the end of a Vector
    while let Some(x) = a.pop() {
        println!("{}",x);
    }

}

fn main() {
    vectors();
}
