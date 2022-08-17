use std::mem;

fn main() {
    //Array, data structure where you KNOW how many elements
    //will be in advance
    let mut a:[i32;5] = [1,2,3,4,5];
    let mut a = [1,2,3,4,5];  
    
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a at 0 has {}",a[0]);

    //Printing entire array
    println!("{:?}",a);

    //You only can compare arrays with the same size
    if a != [1,2,3,4,5]
    {
        println!("Does not match");
    }

    let b = [1; 10];

    for i in 0..b.len() {
        println!("{}",b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3]; 2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    //Values on the Diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}",i ,j ,mtx[i][j]);
            }
        }
    }

}
