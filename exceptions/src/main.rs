use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::{io, fs};

fn main() {
    let err = vec![1, 2, 3];

    let f: Result<File, Error> = File::open("hello.txt");
    f.expect("Fail!");

//    let f = match f {
//        Ok(file) => file,
//        Err(error) =>
//            match error.kind() {
//                ErrorKind::NotFound =>
//                    match File::create("hello.txt") {
//                        Ok(fc) => fc,
//                        Err(e) => panic!("Error creating file {}", e)
//                    },
//                other =>
//                    panic!("Error accessing file {}", error)
//            }
//    };
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
