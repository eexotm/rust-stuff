
#[derive(Debug)]
enum Types {
    int(i32),
    string(String),
    float(f64),
}


fn main() {
    let mut v = vec![1,2,3];
    let mut v2: Vec<String> = vec![String::from("Hello"), String::from("World")];
    v.push(5);
    v2.push(String::from("2"));
    let first = v2[1].clone();
    let second = v[2];
    dbg!(&v);
    dbg!(&v2);
    match v2.get(3) {
        Some(n) => println!("{n}"),
        None => println!("No value"),
    }

    for i in &mut v {
        let m = *i +50;
        println!("{m}");
    }
    let mut v3 = vec![Types::int(32), Types::string(String::from("Nigga"))];
    for i in &mut v3 {
        println!("{:?}", i);
    }
}
