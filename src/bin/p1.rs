fn main() {
    let mut x = 5;
    println!("{x}");
    {
        let x = x + 5;
        println!("{x}");
    }
    println!("{x}");
    let spaces = "    ";
    let spaces = spaces.len();
    let x1:u32 = "42".parse().expect("not a number");

    let arr = [1,2,3,4,5];
    another_function(5);
    // let x = (let y = 5);  doesnt compile because statments reutrn ()
    let z = {
        let y = 5;
        y + 5
    }; // these are BLOCKS they act like funciton blocks and return the last value wihtout ;
}
fn another_function(x:i32) -> () {
    println!("value: {x}");
}