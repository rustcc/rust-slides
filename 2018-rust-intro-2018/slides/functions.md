##  Funciones

Ya has visto una de las funciones más importantes en el lenguaje: la función `main`, que es el punto de entrada de muchos programas. También ha visto la palabra clave fn, que le permite declarar nuevas funciones.

```
fn main() {
    println!("Hola, mundo!");

    mi_funcion();
    mi_edad(20);
}

fn mi_funcion() {
    println!("Otra funcion.");
}

fn mi_edad(edad: u8) {
    println!("Tu edad es: {}", edad);
}
```
