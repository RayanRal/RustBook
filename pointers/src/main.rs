use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem;
use std::rc::Rc;

fn main() {
    smart_pointers();
    custom_dropping();
    rc_pointer();
}

fn smart_pointers() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    hello(&MyBox::new(String::from("world")));
}

fn custom_dropping() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("custom pointers created");
    mem::drop(c);
    println!("custom pointers dropped before end");
}

fn rc_pointer() {
    let a = Rc::new(Cons(5,
                         Rc::new(Cons(10,
                            Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after dropping c = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Rc<List>),
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

//custom dropping functions with Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

