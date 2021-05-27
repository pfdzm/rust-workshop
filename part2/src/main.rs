use std::fmt;

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

fn new_point(x: f32, y: f32, z: f32) -> Point {
    Point { x, y, z }
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: format!(
                "a *non-coastal* city of approximately {} residents",
                residents
            ),
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let rustville: City = new_city(133742069, false);
    let point_a = new_point(12.1, 12.33, 0.2);

    println!("This city can be described as: {}", rustville.description);

    println!("This city has {} residents", rustville.residents);

    println!("Your point is {}", point_a);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
