fn use_slice(slice: &[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
}

fn use_mutable_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 12345;
}

fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&data[1..4]);
    use_mutable_slice(&mut data[1..4]);
    
    println!("{:?}", data);
}

fn main() {
    slices();
}
