
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            length: 8,
        };
        let smaller =  Rectangle {
            width: 5,
            length: 1,
        };

        assert!(larger.can_hold(&smaller))
    }
}

fn main() {
    println!("helo")
}

//44829197