




/* fn longest(x: &str, y:&str ) -> &str {
    if x.len() > y.len() {x} else {y}
}
    */

/*  we return a reference &str, but compiler doesnt know 
if its gonna come from x or y 

&i32         → reference
&'a i32      → reference with lifetime 'a
&'a mut i32  → mutable reference with lifetime 'a

*/

fn longest<'a>(x: &'a str, y:&'a str ) -> &'a str {
    if x.len() > y.len() {x} else {y}
}
struct Important<'a> {
    part: &'a str,

}

impl<'a> Important<'a> {
    fn level(&self) -> i32 { 3 }
}
fn main() {
let string1 = String::from("abcd");

{
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("{result}");
    // result dies when the shortest of the two inputs die, which is string2.
    // result is bound to string2.
}

let abdc = String::from("wazzupo");
{ let string3 = abdc.as_str();
let imp1 = Important {
    part: string3,
};
println!("{}", imp1.part)}
// println!("{}", imp1.part);

}
