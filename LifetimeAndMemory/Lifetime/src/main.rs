struct Person {
    name: String
}

impl Person {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}
/*
struct Company {
    name: String,
    ceo: &Person
}
 */

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

fn main() {
    //Static variables don't have a lifetime &'static str
    //They will live as long as the program is running

    /*
    This won't compile because Rust has to be sure that as
    long as the Company exists the Person will exist too

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss};

    The CEO lifetime and the Company lifetime will be the same
    */

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss};

    let mut z : &String;
    {
        let p = Person { name: String::from("John")};
        z = p.get_ref_name();
    }

}
