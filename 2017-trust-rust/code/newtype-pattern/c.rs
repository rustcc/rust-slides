#[derive(Debug,Clone,Copy)]
struct Celsius(f64);

#[derive(Debug,Clone,Copy)]
struct Fahrenheit(f64);

fn regulate_heating(temperature: Celsius) {

}

fn main() {

    let t = Fahrenheit(90.0); // compilation error!
    regulate_heating(t);

}
