mod restaurant; // Look for restaurant.rs or restaurant/mod.rs

use crate::restaurant::seated::take_order;

fn main() {
    let person = take_order;
    person(); // Calling it
}
