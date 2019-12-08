title: Beginner Rust Workshop
class: animation-fade
layout: true

<!--
This work is licensed under the Creative Commons Attribution-ShareAlike 4.0
International License. To view a copy of this license, visit
http://creativecommons.org/licenses/by-sa/4.0/ or send a letter to Creative
Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

.bottom-bar[
  {{title}}
]

---

class: impact

# {{title}}

.left-column[
Michal Vaner (michal.vaner@avast.com)
]

---

# Materials

* Slides
  - With lists of suggested libraries, hints...
  - https://vorner.cz/tmp/workshop.html
* Working code:
  - Spoiler alert!
  - You can get inspired if you want, but prefer try first
  - https://git.int.avast.com:vaner/duper

---

# The task

* Look through a directory structure
* Find and list duplicate files
  - Based on content
* Start with the bare minimum, extend as times goes
* Possible extensions
  - Error handling/recovery + logging
  - Accepting command line arguments
  - Machine-readable output
  - Speeding it up (multiple threads, early pre-check)

---

# Tips ‒ useful libraries (basics)

* Searching for libraries:
  - https://crates.io
* Documentation:
  - https://docs.rs
* **Hashing**:
  - [`md5`](https://docs.rs/md5) (probably the most friendly API)
  - [`sha2`](https://docs.rs/sha2)
* **Finding files**:
  - [`walkdir`](https://docs.rs/walkdir)

---

# Tips ‒ core data structures

* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
  - It's [entry API](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
  - Iterators
* [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
  - A growing array
  - For storing paths with the same hash

---

# Tips ‒ Error handling

* [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
* The `?` operator
* „Universal“ error type
  - `type Error = Box<dyn std::error::Error + Send + Sync>`
  - [`failure`](https://docs.rs/failure)
* [`log`](https://docs.rs/log) + [`env_logger`](https://docs.rs/env_logger)
  - Logging

---

# Tips ‒ Command line parsing

* [`clap`](https://docs.rs/clap) ‒ Manual handling
* [`structopt`](https://docs.rs/structopt) ‒ Automagic parsing to structure

---

# Tips ‒ Machine readable output

* [`serde`](https://serde.rs) + [`serde_json`](https://docs.rs/serde_json)

---

# Tips ‒ parallelization

* Multiple approaches
* Threads in `std`, a bit awkward for this use case
* [`crossbeam_utils`](https://docs.rs/crossbeam_utils) +
  [`crossbeam_channel`](https://docs.rs/crossbeam_channel) ‒ scoped threads,
  channels
* [`rayon`](https://docs.rs/rayon) ‒ parallel iterators
* [`scoped-pool`](https://docs.rs/scoped-pool) ‒ a threadpool/task executor
