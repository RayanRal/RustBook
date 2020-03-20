
pub(crate) fn refs() {
    let mut x = String::from("hello");
    let x_len = calc_len(&mut x);
    let x_len2 = calc_len(&mut x);
    println!("{}, {}", x, x_len)
}

fn calc_len(s: &mut String) -> usize {
    s.push_str("11");
    return s.len()
}

pub(crate) fn slice() {
    let mut s = String::from("Hello world");
    let slice = &s[..2];
    s.clear();

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    return &s[..s.len()];
}
