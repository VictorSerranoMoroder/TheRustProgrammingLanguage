fn main() {
    let vec = vec![3,2,1];

    //if you do this you will move the vec variable
    //for x in vec

    for x in &vec
    {
        println!("{}", *x);
    }

    //Or we can use an iterator
    for x in vec.iter()
    {
        println!("we got {}", *x);
    }

    //But if we want to modify the x variable or the vector
    let mut mutvec = vec![3,2,1];

    for x in mutvec.iter_mut()
    {
        *x  += 2;
    }
    println!("{:?}", mutvec);

    //We can also run through the vector in reverse order
    for x in mutvec.iter().rev()
    {
        println!("in reverse {}", x);
    }
    
    //Transforming a collection into an iterator
    //This will move the collection to the iterator

    let mut mutvec2 = vec![1,2,3];
    //Behind the scenes...
    //let it = vec.into_iter();
    mutvec2.extend(mutvec);
    println!("{:?}", mutvec2);
    //This won't compile as we have moved the mutvec variable
    //println!("{:?}", mutvec);

}
