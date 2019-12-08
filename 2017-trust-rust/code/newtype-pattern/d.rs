use std::convert::{From, Into};

#[derive(Debug,Clone,Copy)]
struct Celsius(f64);

#[derive(Debug,Clone,Copy)]
struct Fahrenheit(f64);

impl Into<Celsius> for Fahrenheit {
    fn into(self) -> Celsius {
        Celsius((self.0 - 32.0) * (5.0/9.0))
    }
}

fn regulate_heating(temperature: Celsius) {
    println!("{:?}", temperature);
}

fn main() {

    let t = Fahrenheit(90.0); // compilation error!
    regulate_heating(t.into());

}
