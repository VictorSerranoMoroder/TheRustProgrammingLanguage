#[cfg(test)]
mod tests
{

extern crate phrases;

#[test]
#[should_panic] //Expected Failed test
#[ignore]  //We don't run this test now
fn english_greeting_correct() {
    assert_eq!("hello", phrases::greetings::english::hello());
}
}