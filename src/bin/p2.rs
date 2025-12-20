fn main() {

  let s = String::from("HELLO");  
  let x  = 5;
  let y = x;
  println!("{x}{y}");
  let z = String::from("ngg");
  let r = z;
  let n = r.clone();
  println!("{r}");
  println!("{n}");
  let m = take_ownership(n);
    println!("{m}");


  let mut s1 = String::from("hello");
  let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");
    modify_borrow(&mut s1);
    let mut r1 = &s1;
    s1.push_str("hello");
    println!("{s1}");
    
}
fn modify_borrow(s:&mut String) {
    s.push_str(", world");
}


fn take_ownership(s: String) -> String {
    s
}   

fn create_copy(int: i32) -> i32 {
    int
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn len_of_string(s:&str) -> usize {
    s.len()
}