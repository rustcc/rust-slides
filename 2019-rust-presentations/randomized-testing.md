title: Let the computer write tests for you
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

# About the talk

* Introducing some testing techniques
* We want to protect people
  - Not put them at greater risk
  - We want to be sure our code works
  - So we write tests, but…

---

# Writing tests is hard

* Every program contains at least one bug 🐛
  - Including the examples in this talk!
* Nobody *likes* writing tests
* Nobody has *time* to write tests
* Humans are *terrible* at writing tests

???

* Don't spoil it to others and don't point the bugs out

---

# Story of the Monday Morning

* It's Monday morning
* There was a party last night 🍺
* The coffee didn't kick in yet ☕
* There was a fight over the scrambled eggs at the canteen
* A merge request with qicksort implementation came
* Let's see the code

???

* Feel into the situation
* Now let's decide if this code can go in

---

```rust
pub fn qsort(data: &mut [u8]) {
    if data.len() <= 1 { return; }
    let mut l = 0;
    let mut r = data.len() - 1;
    let pivot = data[(l + r) / 2];

    while l < r {
        if data[l] < pivot {
            l += 1;
        } else if data[r] > pivot {
            r -= 1;
        } else {
            data.swap(l, r);
            l += 1; r -= 1;
        }
    }
}
```

---

# Tests passes

```rust
#[test] fn sort_empty() {
    let mut d = [];
    qsort(&mut d);
    assert!(d.is_empty());
}
```

* `[]`
* `[42]`
* `[1, 2, 3]`
* `[3, 2, 1]`
* `[4, 2, 3, 1]`
* `[1, 8, 3, 7, 5, 6, 4, 2, 9]`

???

* Actually run them
* Show the real code

---

# Review checklist

* It is readable
* Looks quicksorty ‒ pivot, swapping, etc
* Follows style guide & naming conventions
  - Quicksort conventionally uses `l` & `r`
* Has tests
  - 100% test coverage
  - Tests edge-cases
  - Tests some larger input
  - All tests are green
* LGTM! 🐙

---

# But it's broken! 🤦

* Doesn't recurse into the halves
* Can go into an infinite loop
* Can go into infinite recursion
* None of the tests noticed

???

* Show the code again?

---

# Introducing property-based testing

* The computer generates test case data 🎲
  - Structured data
  - The author may specify a strategy
  - A *lot* of test cases are run
* The author feeds the data to algorithm
* Then some *properties* of solution are checked
  - Implicit: doesn't crash
  - Often verifying solution is simpler than finding it
  - Possible to compare to trivial (slow) implementation
* If it fails:
  - The harness tries to *minimize* the input
  - Stores the seed, to re-check it next time 💾

???

* Comparing to slow implementation is called model based implementation

---


```rust
proptest! {
*   #[test] fn sort(mut data: Vec<u8>) {
        let original = data.clone();

*       qsort(&mut data[..]);

        prop_assert_eq!(original.len(), data.len());
        for val in &original {
            prop_assert!(data.contains(val));
        }
        for (prev, cur) in data.iter().tuple_windows() {
            prop_assert!(prev <= cur);
        }
    }
}
```

???

* Run it and show it fails
* Do we want to fix it? Maybe at the end of session?

---

# Pros & Cons

* Less work to write than thinking of hundreds of test cases
* Good at spotting overlooked possibilities
  - Likely overlooked in tests too
* More CPU intensive ‒ takes longer to run
* Isn't good at finding edge-cases
* Suitable at finding algorithmic bugs
  - For „small“ things
  - Unit-test scope
* Doesn't replace traditional unit tests, complements them

???

* Real world example in urlite?
* But bunch of other test cases too

---

# Libraries

* Rust: proptest, quickcheck, model
* Haskell: quickcheck
* C++: RapidCheck
* Python: Hypothesis

---

# Next example: Morse decoder

* Using base-2 arithmetics
  - `.` is 0, `-` is 1
  - Prefix each letter with `1` to disambiguate (`.` vs `..`)
* And a lookup in a table
* Let's see the code

---

```c
static const char *tbl = " ETIANMSURWDKGOHVF?L?PJBXCYZQ??54?3???2"
                         "???????16???????7???8?90";

void demorse(char *buff) {
  size_t acc = 1;
  char *write = buff;
  for (char *c = buff; *c; c ++) {
    switch (*c) {
      case '.': acc *= 2; break;
      case '-': acc = acc * 2 + 1; break;
      default:
        *write ++ = tbl[acc - 1];
        acc = 1;
    }
  }
  *write = '\0';
}
```

???

* Who sees the bug?
* HeartBleed style vulnerability if ever exposed to untrusted input
  - The client can ask for a byte arbitrarily out of bounds

---

```c
static const char *tbl = " ETIANMSURWDKGOHVF?L?PJBXCYZQ??54?3???2"
                         "???????16???????7???8?90";

void demorse(char *buff) {
  size_t acc = 1;
  char *write = buff;
  for (char *c = buff; *c; c ++) {
    switch (*c) {
      case '.': acc *= 2; break;
      case '-': acc = acc * 2 + 1; break;
      default:
*       *write ++ = tbl[acc - 1];
        acc = 1;
    }
  }
  *write = '\0';
}
```

---

# Fuzzing

* Tries to feed the program with random inputs
  - Something valid enough to get in
  - Invalid enough to cause problems
* Attempts to trigger misbehaviour
  - Crash
  - Usually run with sanitizers
* Uses different techniques to generate „interesting“ inputs
  - Mutations of provided valid inputs
  - Guided by watching coverage

---

```c
extern int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
  // 0-terminate the string
  char buf[size + 1];
  memcpy(buf, data, size);
  buf[size] = '\0';

* demorse(buf);

  return 0;
}
```

```sh
clang -g -O1 -std=c99 -fsanitize=fuzzer,address \
  morse.c fuzz.c -o fuzz
```

???

* Show the example
* Run it

---

# Pros & Cons

* Good for finding vulnerabilities
  - For code handling untrusted inputs
  - Like parsers, decoders, …
* Learns how to „get in“
  - By tracking the coverage
  - Learns how the input looks like
* Heavy-weight compared to the property based testing
* Very CPU intensive
  - May need to run for days or weeks ⏰

???

* Eg. learns how an URL looks like
* Or, as we see, that it needs to add more dots

---

# Tools

- Generally, many
  * This is an old technique
  * Recent ones push it to more practical levels

* AFL
  - From outside of the process
* LibFuzzer
  - Based on LLVM, uses its sanitizers
  - Relatively easy to use
  - Inside the process
* HongFuzz
  - Can use HW counters instead of instrumentation

---

# What can this be used for?

* Intentionally left empty, fill with your own ideas 😇
