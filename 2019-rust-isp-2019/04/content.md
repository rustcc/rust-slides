# Community & Contributing

---
## Contributing to Rust

- For the final project (or for fun!) you're allowed to contribute to the Rust language project, or
    to several other Rust projects.
- Read the [contributing guide here](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)
    to get started.
- Also check out the [issue tracker](https://github.com/rust-lang/rust/issues).
    - There are 4,568 issues open as of time of writing!
    - Filter by E-Easy, E-Medium, or E-Mentor to find good starting issues.

---
## Contributing to Rust

- There are three(ish) main parts of the Rust project:
    - `rustc`, the Rust compiler
    - `libstd`, the Rust standard library
    - documentation
- If you want to contribute to core `rustc`, beware that you may find it very difficult!
- There's no real distinction between `rustc` & `libstd` in the issue tracker, but it's usually
    pretty obvious what an issue is about.
- If you wish to contribute to Rust for the final project, don't _only_ work on documentation.

---
### Etiquette

- If you wish to contribute, make sure you also read and follow Rust's
    [Code of Conduct.](https://www.rust-lang.org/conduct.html)
- Be polite. Be organized. Don't create unnecessary work for other people.
  Test your code well. Use GitHub properly (issues and pull requests). 

---
### Fork-Pull Request Model

- When you want to work on a feature, start by forking the Rust repo on Github.
- When you've completed whatever feature you're working on, make a pull request from your fork
    against the main Rust repo, and await review.
- You'll be assigned a reviewer by bors, the Rust bot.
- Your reviewer may ask you to make changes, provide more tests, etc. to your code.
- Once review is complete, your changes will be "rolled up" by bors into a changeset and merged into
    `master` when appropriate.

---
### RFCs

- If you want to propose a _larger_ feature for Rust, Cargo, or Crates.io, you should create an
    _RFC_, or Request For Comments.
    - e.g. semantic/syntactic changes, additions to `std`, removing language features...
- Before you submit an RFC, you may want to discuss it on the Rust internals forum, IRC, etc. and
    talk to the core team about it first.
    - This is suggested to help you flesh out ideas & make sure an RFC won't get rejected outright.
- If you think your idea is still worth proposing, write a formal RFC using the template on the RFCs
    repo, following the process [here](https://github.com/rust-lang/rfcs/blob/master/README.md).

---
### Resources

- [Rust Users Forum][(http://users.rust-lang.org/)
- [Rust Internals Forum][(http://internals.rust-lang.org/)
- [/r/rust on Reddit](https://www.reddit.com/r/rust)
- [Discord](https://discord.gg/rust-lang)
- [#rust on IRC](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust) (also check #rust-beginners)
- [This Week In Rust](https://this-week-in-rust.org/) (includes "Call for Participation")

---
## Rust Community Projects

- [Servo](https://servo.org/)
- [Piston](http://www.piston.rs/) and [ggez](http://ggez.rs/)
- [Redox](http://www.redox-os.org/)
- Others

---
### Servo

- A parallel browser engine project developed in Rust by Mozilla.
- [Contributing](https://github.com/servo/servo/blob/master/CONTRIBUTING.md) guide.
- [Quick-Start](https://github.com/servo/servo/blob/master/HACKING_QUICKSTART.md) guide.

---
### Piston and ggez

- A game engine under development in Rust.
- piston: [Contributing](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md), [Quick-Start](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)
- ggez: [Contributing](https://github.com/ggez/ggez/blob/master/CONTRIBUTING.md)

---
### Redox

- A brand-new operating system written in Rust!
- [Contributing](https://github.com/redox-os/redox/blob/master/CONTRIBUTING.md)

---
### Other Projects

- Look at [libs.rs](http://libs.rs/), [Awesome Rust](https://github.com/kud1ing/awesome-rust) and
    [Not-Yet-Awesome Rust](https://github.com/not-yet-awesome-rust/not-yet-awesome-rust) for more ideas.
