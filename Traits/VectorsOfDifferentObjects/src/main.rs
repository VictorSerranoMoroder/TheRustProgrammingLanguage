
trait Animal {
    //static: calle as Animal::create()
    //returns the type of the implementor
    //fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}
struct Human {
    name : &'static str,
}
impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("Hello my name is {}.", self.name());
    }
}
struct Cat {
    name : &'static str,
}
impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk (&self) {
        println!("{} says meow", self.name());
    }
}

//First Approach, Enums
enum Creature {
    Human (Human),
    Cat (Cat)
}

//If the objects are completely different use Enums
fn enumApproach() {
    let mut creatures = Vec::new();
    //creatures.push(Human{name:"John"});
    //creatures.push(Cat{name:"Fluffy"});

    creatures.push(Creature::Human(
        Human{name: "John"}
    ));

    creatures.push(Creature::Cat(
        Cat{name: "Fluffy"}
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c)   => c.talk()
        }
    }
}


//If the objects added share a Trait you can use a Box
fn BoxApproach() {
    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(
        Box::new(Human{name:"Michael"}));
        animals.push(
        Box::new(Cat{name:"Mew"}));

    for a in animals.iter() {
        a.talk();
    }
}

fn main() {
    enumApproach();
    BoxApproach();
}


