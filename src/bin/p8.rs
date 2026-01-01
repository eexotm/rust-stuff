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

pub struct Car {
    pub state: bool,
    pub fuel: i32,
}

pub trait Actions {
     fn start(&mut self) -> ();
    fn check_fuel(&self) -> i32;
 fn add_fuel(&mut self, value:i32) -> ();
    fn print_details(&self) -> String {
        String::from("DETAILS")
    }
}

impl Actions for Car {
    fn start(&mut self) -> () {
        if self.state == true {
            println!("Car is already on")
        }else {
        self.state = true;
        println!("Car is now on")
        }
      }

    fn check_fuel(&self) -> i32 {
        self.fuel
    }
    fn add_fuel(&mut self, value:i32) -> () {
        self.fuel = self.fuel + value;
        println!("Current fuel state {}", self.fuel)
    }
}
struct Truck {
    height: i8,
    fuel: i32,
    state: bool,
}
impl Actions for Truck {
    fn start(&mut self) -> () {
        if self.state == true {
            println!("Truck is already on")
        }else {
        self.state = true;
        println!("Truck is now on")
        }
      }

    fn check_fuel(&self) -> i32 {
        self.fuel
    }
    fn add_fuel(&mut self, value:i32) -> () {
        self.fuel = self.fuel + value;
        println!("Current fuel state {}", self.fuel)
    }
}
pub fn general(item: &impl Actions) {
    println!("Hello {}", item.print_details())
}

pub fn general2<T: Actions>(item: &T) {
    println!("Hello {}", item.print_details())s
}

pub fn general3<T,U>(t: &T, u: &U) where T: Actions, U: Actions, {
    
    
}

pub fn general4() -> impl Actions {
    Car {
        state: false,
        fuel: 45,
    }
}
use std::fmt::Display;
struct Pair<T> {
    x:T,
    y:T,
}
fn main() {
    let p1 = Point {x: 10, y: 0.5};
    let p2 = Point { x: 1.5, y: 2.5};
    p2.addf();

    let mut car  = Car {state: false, fuel: 30};
    car.add_fuel(60);
    car.start();
    println!("{}", car.check_fuel());
    println!("{}", car.print_details());
    let mut truck = Truck {height: 3, fuel: 50, state: false};
    general(&truck);
    
}