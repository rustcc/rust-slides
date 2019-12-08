##  Compilar y ejecutar tu proyecto

Para compilar el codigo ejecutamos el siguiente comando:

```
$ cargo build
```

Cargo creara un archivo ejecutable en `./target/debug/`

```
$ ./target/debug/hello_world
Hello, world!
```

Podemos resumir estos pasos en uno solo con:

```
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.10s
 Running `target/debug/hello_world`
Hello, world!
```
