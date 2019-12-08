% Cargo and Crates
% Pramode C.E
% February 14, 2018



# Cargo

- The Rust package manager and build system

- Library dependencies specified in Cargo.toml


# Crate

- A Rust library/package

- Composed of one or more source files / modules 

- Built using Cargo


# crates.io

- Central repository containing crates

- Anyone can create a crate and upload to crates.io

# 

- Crate names are unique and taken on first-come basis

- Published crates can't be removed/modified

- You can publish as many new versions as you like

# Semantic Versioning (SemVer)

- Crate versions have to follow SemVer rules

- https://semver.org/

 
# Publishing a crate

- Get an account on crates.io and acquire an API token

- Save the token to ~/.cargo/credentials by running the
  command: "cargo login API\_KEY".  


# Packaging the crate

- Use this command: "cargo package" 

- Creates a *.crate file in target/packages

- Check the size of the file

# Upload the crate

- Use "cargo publish" to publish the crate!



