use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error>{
    let file = File::open("hello.txt");

    match file {
        Ok(n) => n,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("my_folder/hello.txt").expect("Unable to create file"),
            _ => panic!("Problem opening the file"),
        }
    };
    File::open("hello.txt")?;
    Ok(())
}


fn read_username() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name)
}