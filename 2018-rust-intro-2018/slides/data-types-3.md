##  Tuplas

Una tupla es una forma general de agrupar algunos otros valores con una variedad de tipos en un tipo compuesto.

```
fn main() {
    let t: (i32, f64, u8, char) = (500, 6.4, 1, 'x');
    println!("({}, {}, {}, {})", t.0, t.1, t.2, t.3);
}
```
