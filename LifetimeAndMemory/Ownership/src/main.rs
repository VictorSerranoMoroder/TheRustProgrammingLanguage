fn main() {
    // V OWNS what is bound to or
    //  it owns the memory that is stored in the Vector

    // V is on the stack while the data is on the heap
    let v = vec![1,2,3];

    //  Only 1 variable owns the memory at the same time
    //  v2 copies the pointer and v is invalidated
    //let v2 = v;

    //This won't compile because the value/pointer has
    //been moved to v2 and v has been invalidated or moved
    //println!("{:?}", v);

    println!("{:?}", v);


    //As soon as we use v in the closure is no longer usable
    //because we moved the vector to the v variable inside the closure
    //let foo = |v:Vec<i32>| ();
    //foo(v);

    //We can return the reference like this so the Ownership
    //is not lost
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        return x;
    }; 

    //But it is very inconvenient to return the Ownership
    //After you have taken over, thats when Borrowing comes in
    let vv = print_vector(v);


    //When using primitives it won't happen because we
    //interpret this assignation as a full copy
    // u2 is not pointing to the same data but
    //a copy of the first one

    let u = 1;  //i32
    let u2 = u;
    println!("u = {}", u);

    //To be sure an object behaves like a primitive when
    //in this case we should implement the trait Copy
}
