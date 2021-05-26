struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
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

    println!("This city can be described as: {}", rustville.description);

    println!("This city has {} residents", rustville.residents);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
