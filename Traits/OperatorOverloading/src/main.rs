
//Operator Overloading in Rust is done by Traits


use std::ops::{Add, AddAssign, Neg};
use std::cmp::{PartialEq};

#[derive(Debug, Eq)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    //Factory Method
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

////THIS ONLY WORKS WITH I32
//impl Add for Complex<i32> {
//    type Output = Complex<i32>;
//
//    //a+b
//    fn add(self, rhs: Self) -> Self::Output {
//        Complex {
//            re: self.re + rhs.re,
//            im: self.im + rhs.im
//        }
//    }
//}

//For Generic Type of addition
impl<T> Add for Complex<T> 
    where T: Add<Output = T>
{
    type Output = Complex<T>;

    //a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    //a+=b
    fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im;
    }

}

impl<T> Neg for Complex<T>
    where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

//  Partial eq
impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }   

    //fn ne(&self, rhs: &Self) -> bool {
    //    
    //}
}

//  Full eq : X = X
//  NAN = Not a number 0/0 inf/inf
//  NAN == NAN -> False
//  We can implement it
//  impl<T : Eq> Eq for Complex<T> where T: Eq 
//  Or Derive it!



fn main() {
    let mut a = Complex::new(1.2,2.4);
    let mut b = Complex::new(3.2,4.7);

    //We need an Add Implementation
    //println!("{:?}", a + b);

    //We need an AddAssign Implementation
    a += b;
    println!("{:?}", a);
    //println!("{:?}", -a);
    println!("{:?}", a==a);

}
