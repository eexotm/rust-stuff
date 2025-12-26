use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic!("Panicked");
    let file = File::open("hello.txt");

    match file {
        Ok(n) => n,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("hello.txt").unwrap(),
            _ => panic!("Problem opening the file"),
        }
    };


}