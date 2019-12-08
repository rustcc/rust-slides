# Intro to Rust Presentation Outline

1. Opening slide
1. How this presentation works. Interactiveness explained. Large credit to RustByExample.com
1. Link to presentation.
1. Goals of presentation.
  1. Things that are covered.
    1. Explain what Rust is and where it comes from.
    1. Offer basic examples of all core aspects of language.
  1. Things that aren't covered.
    1. Cargo.
    1. How to organize projects.
1. What is Rust.
  1. Photo from rustcamp (Rust is X without Y).
1. Learning basics.
  1. Hello world.
  ```rust
  fn main() {
        println!("Hello World!");
  }
  ```
  1. Printing options.
  ```rust
  fn main() {
        println!("{} days", 31);

        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        println!("{subject} {verb} {predicate}",
                 predicate="over the lazy dog",
                 subject="the quick brown fox",
                 verb="jumps");

        println!("{} of {:b} people know binary, the other half don't", 1, 2);

        println!("My name is {0}, {1} {0}", "Bond", "James");
  }
  ```

  1. Printing complex things.

  ```rust
  struct Structure(i32);

  fn main() {
        // Custom types such as this structure require more complicated
        // handling. This will not work.
        println!("This struct `{}` won't print...", Structure(3));
  }
  ```

  1. Printing complex things.

  ```rust
  #[derive(Debug)]
  struct Structure(i32);

  fn main() {
      // Printing with `{:?}` is similar to with `{}`.
      println!("{:?} months in a year.", 12);
      println!("{1:?} {0:?} is the {actor:?} name.",
               "Slater",
               "Christian",
               actor="actor's");

      // `Structure` is printable!
      println!("Now {:?} will print!", Structure(3));
  }
  ```

  1. Custom printing, imports, traits.

  ```rust
  // Import (via `use`) the `fmt` module to make it available.
  use std::fmt;

  // Define a structure which `fmt::Display` will be implemented for. This is simply
  // a tuple struct containing an `i32` bound to the name `Structure`.
  struct Structure(i32);

  // In order to use the `{}` marker, the trait `fmt::Display` must be implemented
  // manually for the type.
  impl fmt::Display for Structure {
      // This trait requires `fmt` with this exact signature.
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          // Write strictly the first element into the supplied output
          // stream: `f`. Returns `fmt::Result` which indicates whether the
          // operation succeeded or failed. Note that `write!` uses syntax which
          // is very similar to `println!`.
          write!(f, "{}", self.0)
      }
  }
  ```


### Unsorted

* Best places to learn.
  * RustByExample.com
* Best tools.
* Releases explained.
* Writing a safe Python extension.
* Rust community.
* Cargo.
* Documentation generation.
* Strengths / Weaknesses.
  * Strengths:
    * Safe.
  * Weaknesses:
    * Complex.
    * Rigid.
