
fn strings() {
    //&str = string slice
    //Inmutable strings 
    let s:&'static str = "Hello There"; 
    //s = "abc"
    //let h = s[0]

    //We can get the chars like this...
    for c in s.chars() {
        println!("{}",c);
    }

    for c in s.chars().rev() {
        println!("{}",c);
    }

    //Safe way of getting a char from a string slice
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    //Heap
    //String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");   //inputs an string slice
        a += 1;
    }
    println!("{}", letters);

    //Conversions
    //&str <> String
    let u:&str = &letters;

    //Concatenation
    //String + str
    //let z = letters + "abc";
    //let y = letters + &letters;

    let mut abc = String::from("hello World");
    abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn main() {
    strings();
}
