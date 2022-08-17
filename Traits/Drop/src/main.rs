struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature{ name: name.into() }
    }
}

/*
The Drop trait only has one method: drop, 
which is called automatically when an object goes out of scope. 
The main use of the Drop trait is to free the resources that the implementor instance owns.
 */

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}



fn main() {
    let mut clever: Creature;
    {
        //Life of goblin goes beyond this encapsulated scope
        //if it is referenced from outside
        let goblin = Creature::new("Jeff");
        println!("game proceeds"); 
        clever = goblin;
        //we can force the end of life of a variable
        //drop(goblin);
        //after this point you can't use the variable
    }

    println!("End of the program");
}
