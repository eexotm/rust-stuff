fn do_something<T>(value: T) -> T{
    value
}

struct Point<T,U> {
    x: T,
    y: U,
}
impl Point<f32,f32> {
    fn addf(&self) -> f32 { //type special function.
        self.x + self.y
    }
}


fn main() {
    let p1 = Point {x: 10, y: 0.5};
    let p2 = Point { x: 1.5, y: 2.5};
    p2.addf();
}