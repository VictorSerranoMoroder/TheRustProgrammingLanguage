use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);

    println!("a square has {} sides",
        shapes["square".into()]);

    //Print HashMap
    for (key,value) in &shapes  {
        println!("{} : {}", key, value);
    }

    //Modify things in the HashMap
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    //If the key isn't there we don't do nothing
    //If the key is there we DONT modify anything
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);


    //We can modify values found doing this
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(1);
        *actual = 0;
    }
    println!("{:?}", shapes);


}
