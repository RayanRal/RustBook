
pub(crate) fn iters() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    for val in v2.iter() {
        println!("Got {}", val)
    }
}