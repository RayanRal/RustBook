const VALUE: i32 = 100;

mod ownership;
mod references;
mod structs;
mod enums;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of VALUE is: {}", VALUE);

    let guess: u32 = "42".parse().expect("Error");
    println!("{}", guess);

    let tup: (u32, i32) = (500, 2);

    let (x, y) = tup;

    let a = [3; 5];

    another_function(5);

    let a = [5, 4, 3, 2, 1];
    for i in (1..4).rev() {
        println!("{}", i)
    }

//    ownership::func()
//    references::refs();
    references::slice();
}

fn another_function(x: i32) -> i32 {
    println!("Another!");
    return x;
}
