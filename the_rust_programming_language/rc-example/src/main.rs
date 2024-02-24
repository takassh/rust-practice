#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

impl Drop for List {
    fn drop(&mut self) {
        println!("dropped : {:?}", self);
    }
}

fn main() {
    // You don't need to make let for Nil and Cons(10, Rc::new(Nil))
    // It's automatically freed when every owner is dropped
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}


//// Similar behavior but it's different. See let n and let i
// #[derive(Debug)]
// enum List<'a> {
//     Cons(i32, Box<&'a List<'a>>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// impl<'a> Drop for List<'a> {
//     fn drop(&mut self) {
//         println!("dropped : {:?}", self);
//     }
// }

// fn main() {
//     // You need to declare n and i
//     let n = Nil;
//     let i = Cons(10, Box::new(&n));
//     let a = Cons(5, Box::new(&i));
//     let b = Cons(3, Box::new(&a));
//     let c = Cons(4, Box::new(&a));
// }