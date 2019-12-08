struct Rustacean {
    name: String,
    height: f64,
    weight: f64,
}

trait Fighter {
    fn stats(&self) -> String;
}

impl Fighter for Rustacean {
    fn stats(&self) -> String {
        format!(
            "Name: {}, Height: {}, Weight: {}",
            self.name, self.height, self.weight
        )
    }
}

fn main() {
    let tom = Rustacean {
        name: String::from("Tom"),
        height: 10.2,
        weight: 300.3,
    };
    println!("{}", tom.stats());
}
