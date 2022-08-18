struct Person<'a> {
    name: &'a str
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, {}", self.name);
    }
}

fn main() {
    let person = Person{ name: "Dunno"};
    person.talk();

}
