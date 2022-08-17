fn main() {
    
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x);
        //x.push(123);
    };

    let v = vec![3,2,1];
    //Borrows the reference to the function
    print_vector(&v);
    println!("{:?}",v);

    //There can be many references to a resource but
    //Only one mutable... Maybe fixed in later versions
    let mut a = 40;
    {
        //The borrow has to match the mutability
        let b = &mut a;
        *b *= 2;
    }
    //Access the reference
    //now a is borrowed by b so a cannot be accesed
    println!("a = {:?}", a);  //Well... the compiler returns the ownership


    let mut z = vec![1,2,3];

    for i in &z {
        println!("i = {}", i);
        //If you are iterating a vector you cannot
        //borrow it because it is already borrowed with i
        //z.push(5);
    }
}
