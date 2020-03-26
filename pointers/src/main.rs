use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem;

fn main() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello(&MyBox::new(String::from("world")));

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("custom pointers created");
    mem::drop(c);
    println!("custom pointers dropped before end");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

//smart pointers
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

