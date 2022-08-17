//32 Bits
union IntOrFloat
{
    i:  i32,
    f:  f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe{
        match iof{
            IntOrFloat { i: 1} => {
                println!("Statement 1");
            }
            IntOrFloat { f }   => {
                println!("Value is float {}", f);
            }
        }
    }
}

//In unions you don't know what value u had assigned
//So you need a unsafe block to get values from them

fn main() {
    let mut iof = IntOrFloat { i: 123};
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);
    process_value( IntOrFloat { i: 5 });    //Will interpret as float
}
