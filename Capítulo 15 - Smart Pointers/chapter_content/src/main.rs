fn main() {
    //Using Box<T> to Point to Data on the Heap:

    let b = Box::new(5);
    println!("b = {b}");
    // Enabling Recursive Types with Boxes:
    ex2() ;
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::ops::Deref;

use crate::List::{Cons, Nil};
fn ex() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

//Treating Smart Pointers Like Regular References with Deref:
//Defining Our Own Smart Pointer:
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn ex2() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

//Implementing the Deref Trait:

impl <T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}