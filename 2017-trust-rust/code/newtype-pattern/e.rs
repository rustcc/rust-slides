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

fn regulate_heating<T:Into<Celsius>>(temperature: T) {
    let celsius = temperature.into();
    println!("celsius = {:?}", celsius);
}

fn main() {

    let t1 = Fahrenheit(90.0); 
    let t2 = Celsius(90.0); 
    regulate_heating(t1);

}
