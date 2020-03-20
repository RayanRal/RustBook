
pub(crate) fn func() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let mut x = s.clone();
    x.push_str("!!!");
    println!("{}", x);

    let new_assignment = String::from("hell");
    let f_result = foo(new_assignment);
//    println!("{}", new_assignment);
}

fn foo(a: String) -> String {
    return a + "o";
}