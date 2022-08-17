fn is_even(x: u32) -> bool {
    x % 2 == 0
}

//Generator
fn greater_than(limit : u32) 
    -> impl Fn(u32) -> bool
{
    //move is for extending limit lifetime
    move |y| y > limit
}

fn HighOrderFunctions() {
    //sum of all even squares < 500

    let limit = 500;
    let mut sum = 0;

    //let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i*i;

        if above_limit(isq) {
            break;
        }
        else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);


    //High Order Value
    let sum2 = (0..)
        .map(|x| x*x)                   //Takes a value and transforms it
        .take_while(|&x| x < limit)     //Will only take numbers from the sequence until...
        .filter(|x:&u32| is_even(*x))   //This function returns only those numbers that are even
        .fold(0, |sum, x| sum + x);     //Folds the sequence into a single value

    println!("hof sum = {}", sum2);
}

fn main() {
    //functions that take functions
    //f(g) { let x = g();}

    //functions that return functions
    //Called generators

    //f() -> g

    HighOrderFunctions();
}
