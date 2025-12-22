pub mod restaurant {
    pub mod in_line {
        pub fn add_to_waitlist() {}
    }
    pub mod seated {
        pub fn take_order() {}
    }
}


use crate::restaurant::seated::take_order;
fn main(){
    let person = take_order;
}