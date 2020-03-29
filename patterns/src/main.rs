
fn main() {
    iflet();
    let point = (5, 6);
    print_coordinates(&point);
    destructuring();
    conditions();
    at_bindings();
}

fn iflet() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, _, y) = (1, 2, 3);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}


struct Point {
    x: i32,
    y: i32,
}

fn destructuring() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point {x, y: 0} =>
            println!("On the x axis at {}", x),
        Point {x: 0, y} =>
            println!("On the y axis at {}", y),
        Point {x, y} =>
            println!("On neither axis: ({}, {})", x, y),
    };
}

fn conditions() {
    let num = Some(5);

    match num {
        Some(x) if x < 4 => println!("less than four: {}", x),
        Some(x) => println!("Some: {}", x),
        None => println!("None"),
    }
}

enum Message {
    Hello { id: i32 },
}

fn at_bindings() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}


