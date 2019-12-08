# Intro

* Who I am and warn about the bias
  - Experience both with C++ and Rust
  - Maybe both communities will hate me for it
* Why talking about Rust to C++ folks
  - Why is C++ so popular? It allows to do stuff impossible in other languages.
  - What other languages allow us to do that? C and now Rust ‒ solves the same
    problems.
  - Rust is inspired by C++ ‒ product of C++
  - Practices learned in one can sometimes be useful in the other
* About the talk in general
  - Mostly intuitive
  - No time for fine details as written in the standard
  - Rust doesn't have a standard
  - We'll talk about real-life implementations of C++
* The same execution model (almost)
  - Heap, static storage, several threads with their stacks
  - No GC, (semi) manual memory management
  - Ahead-of-the-time compiled to native, heavily optimized
  - Structures compose without indirection, inline
  - Threads map to OS threads (but allow building on top)
  - Memory orderings
* What this brings:
  - Similar performance characteristics
  - C ffi is very cheap (other-language extensions, dylibs, static libs, …)
  - The same runtime-examination tools work (gdb, perf, heaptrack,
    dissasemblers, …)
  - Ability to call each other cheaply
  - Cross-language optimizations (eg. inlining between languages)
* Unsorted the same
  - RTTI
  - Zero-overhead abstractions
  - Dynamic + static dispatch
  - „object“ model
  - Allows no-std mode
  - Compile-time meta-programming
  - Both have undefined behaviour (but Rust doesn't have infinite loops there)
  - Compile times
* Features:
  - Cargo
  - Cargo update/outdated, cargo audit
* Differences:
  - No exceptions
  - No inheritance
  - No type auto-conversions
  - No overloading
  - No default parameters
  - No pointer arithmetics (only as a method)
  - Hellfire combos
  - Checked generics
  - Attitude towards programmer & safety & readability
  - Build systems
  - Big vs. easy to include libraries
  - Error messages


## References/sources

C++ runtime model
https://www.youtube.com/watch?reload=9&v=JaKKzgqINhI&feature=youtu.be

## Materials

https://twitter.com/graydon_pub/status/958192076209897472/photo/1

Basically I've an anxious, pessimist personality; most systems I try to build
are a reflection of how terrifying software-as-it-is-made feels to me. I'm
seeking peace and security amid a nightmare of chaos. I want to help programmers
sleep well, worry less.
