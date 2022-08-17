trait Animal {

    //Statics funtions don't have themselves as parameter
    fn create(name: &'static str) -> Self;

    //Abstract functions
    fn name(&self) -> &'static str;
    //Default function that can be overrided
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str 
}

impl Animal for Human {
    
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }

    fn name (&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} can talk", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name: name}
    }
    fn name (&self) -> &'static str {
        self.name
    }
}


trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {result += *x;}
        return result;
    }
}

fn traits() {
    //let h = Human{name: "John"};
    //h.talk();
    let h = Human::create("John");
    h.talk();
    let c = Cat{name: "MIsty"};
    c.talk();

    let h2:Human = Animal::create("Michael");
    h2.talk();

    //You can add traits to types you haven't created yourself
    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

fn main() {
    //Defines relations between structs
    traits();
}
