title: Atomics for Human Beings
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

???

* Welcome, introductions.
* Mention experience ‒ arc-swap, contrie
* Allow for questions during the talk.

---

# Plan of the talk

* Anatomy of atomics:
  - What they are.
  - What problems they solve.
  - What problems they pose.
* Explanation of Orderings.
  - With some safe usage patterns.
* TODO: Link to the HTML version

---

# What is an Atomic?

* Special variables:
  - Can be *safely* modified from multiple threads.
  - Doesn't need a Mutex.
* In `std::sync::atomic`.
* Only some types know how to be atomic.
* The same RAM, different code to access.
* **Warning:**:
  - Easy to use incorrectly.
  - Doesn't follow intuitive laws of causality.
  - If unsure, use `Mutex`.

???

* Before the description what an atomic is.
* Q: Who likes solving hard problems?
* Q: Who likes their programs to run fast?

* Motivation for the talk:
  - Atomics are building blocks of mutexes.
  - Atomics are building blocks of lockfree data structures.
  - There allow finer optimizations.
  - There pose interesting problems.
  - Therefore we want to play with them.

---

# Broken Example

```rust
let mut cnt = 0;
crossbeam_utils::thread::scope(|s| {
    for _ in 0..4 {
        s.spawn(|_| {
            for _ in 0..1000 {
                cnt += 1;
            }
        });
    }
}).unwrap();
println!("{}", cnt);
```

???

* Q: How much will it print?

--
* Won't compile.

---

class: broken

# Even More Broken Example

```rust
let mut cnt = 0usize;
crossbeam_utils::thread::scope(|s| {
    let cnt = &mut cnt as *mut usize as usize;
    for _ in 0..4 {
        s.spawn(move |_| {
            for _ in 0..1000 {
                unsafe { *(cnt as *mut usize) += 1 };
            }
        });
    }
}).unwrap();
println!("{}", cnt);
```

???

* Hammering the compiler with unsafe until it finally compiles.
* It compiles, but what does it print?

--
* *Sometimes* 4000. Sometimes less.
* Contains Undefined Behaviour (a data race).

???
* Run the program multiple times. Without --release.
* Yes, it could also eat your kitten, or explode, or send 1001 emails.

---

# What happens

* People want programs to run fast.
* Optimizations on all levels:
  - Memory caches.
  - Computations in registers.
  - Compiler reorderings.
  - All of them *assume* single-threaded execution.
* RAM computation model is *inacurate*.

--
* Each thread has its own *idea* what the memory contains.
  - And it's own idea in what order operations happen.
* Need to be told explicitly to tell other threads.
  - Otherwise the information might or might not leak on its own.

---

# Naïve model

.center[
![Naive RAM model](atomics/naive-ram.svg)
]

---

# Reality (almost)

.center[
![Cached RAM model](atomics/caches-ram.svg)
]

---

# The Memory Model

* Taken from C++/LLVM.
* Very dense and formal definition.
  - Happens-before, synchronizes-with, data-access, …
* [LLVM explanation](https://llvm.org/docs/LangRef.html#memory-model-for-concurrent-operations).
* Necessary when writing proof of correctness.
  - Good idea to do in more complex cases.
* Not useful in simple cases or design phase.
* Defines `Ordering`:
  - Levels of how much to tell other threads.
  - Per access.

???

* The Ordering is per access, but it's better to designate a level for the whole
  variable.
* The higher ordering guarantees, the more expensive it is.
  - On some platforms, some might get promoted.
  - Some operations increase the cost, eg. read-write operation is the same cost
    on AcqRel as SeqCst on x86-64.

---

# The Memory Model

.center[
![Thread model](atomics/model.svg)
]

---

# Ordinary variables ‒ unordered

* Don't know in what order operations happen.
  - Variables „get confused“ by multiple threads.
* UB when not synchronized by other means.

.center[
![Unordered gets confused](atomics/unordered.svg)
]

???

* All the usual variable, not part of ordering. By far the fastest to use.
* The above broken example.
* Either read only & shared, or owned by single thread.

---

# I don't care about others ‒ `Relaxed`

* Each one has its own, independent, timeline.
  - No relation to other variables or Atomics.
* Good for counting things.
* Good for metrics.
  - If you don't mind there might be more responses than requests.
* Good for claiming tokens.

.center[
![Relaxed](atomics/relaxed.svg)
]

???

* Doesn't really exist on x86-64, promoted to AcqRel.

---

# I don't care about others ‒ `Relaxed`

```rust
let cnt = AtomicUsize::new(0);
crossbeam_utils::thread::scope(|s| {
for _ in 0..4 {
    s.spawn(|_| {
	for _ in 0..1000 {
	    cnt.fetch_add(1, Ordering::Relaxed);
	}
    });
}
}).unwrap();
println!("{}", cnt.load(Ordering::Relaxed));
```

???

* Metrics work in a similar way.

---

# Beware of unrelated atomics

* Order of changes from different atomics is unspecified.
* Can differ from thread to thread.

.center[
![Unrelated relaxed](atomics/relaxed-unrelated.svg)
]

???

* This reordering can happen only on *different* atomics.
* Order of each separate atomic is always preserved.

---

class: broken

# Broken example

```rust
// Thread 1
value.store(42, Ordering::Relaxed);
ready.store(true, Ordering::Relaxed);

// Thread 2
while !ready.lead(Ordering::Relaxed) {
}
// Boom: This can fail.
assert_eq!(42, Ordering::Relaxed);
```

* Not an UB.
* *Just* doesn't work correctly.

---

# I'll tell you all I know ‒ `Release`/`Acquire`

.left-column[
* `Release` *tags* a written value.
  - Snapshot of the memory content.
* Reading with `Acquire` receives the snapshot.
* `AcqRel` ‒ combination of both.
* **Warning**:
  - `Acquire` needs a read.
  - `Release` needs a write.
  - Failed `compare_and_swap` *doesn't* write.
* Still independent timelines of Atomics.
]

.right-column[
![Release Acquire](atomics/acq-rel.svg)
]

???

* The timeline of the atomic is a wormhole between the parallel thread's
  universes. The release-acquire piggy-backs a message about the universe state,
  but it needs the value to ride on.

---

# `Release`/`Acquire` ‒ Spinlock

```rust
while locked.swap(true, Ordering::Acquire) {
    // Got true - it's locked by other thread.
    // Waste some time by doing nothing and try again.
}

// It is locked here and we can play with stuff.
// See UnsafeCell for mutable access into something.

locked.store(false, Ordering::Release);
```

???

* Acquire ‒ we want all the changes the previous owner of the lock did.
* Release ‒ Make sure the next one to lock it gets all our changes.
* Want something inside, wrap it in nice API, better parking...
* Releasing without previous acquire can break a chain.

---

# `Release`/`Acquire` ‒ Sending a message

```rust
let channel: AtomicPtr<Message> =
    AtomicPtr::new(std::ptr::null_mut());

// Send a message (assuming it's currently null)
let msg = Box::new(Message::new("Hello"));
*let old = channel.swap(Box::into_raw(msg), Ordering::Release);
assert!(old.is_null());


// Receiving the message in another thread
*let msg = channel.swap(std::ptr::null_mut(), Ordering::Acquire);
if !msg.is_null() {
    let msg = unsafe { Box::from_raw(msg) };
    msg.process();
}
```

???

* This is very contrived example
* Release to publish the just-created message
* Acquire to get the content of the message
* Relaxed would get the *value* of the pointer, but not the *pointee*.
* Interesting problems about ownership, who frees what and when, etc.
* Crossbeam-epoch can help.

---

# Unifying timelines with `SeqCst`

* Like `AcqRel`.
  - Including all the ways it *doesn't* work.
* The operation is additionally anchored on a single global timeline.
  - In addition to Atomic's *private* timeline.
  - Useful if multiple atomics are in play.
* There are no *simple* uses of `SeqCst`.
  - If you are not sure, use a `Mutex`.
  - If you *think* you need `SeqCst`, you need a proof.

???

* The usual saying is „If you are not sure, use SeqCst“.
* But it's either trivial, then you can use the above ones.
* Or it's complex and you don't know if SeqCst is *enough*.

---

# Further information

* C++ has `Consume` as well:
  - Hard to reason about.
  - Compilers don't know how to use properly.
* [Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html).
* [LLVM](https://llvm.org/docs/LangRef.html#memory-model-for-concurrent-operations).
* [nomicon](https://doc.rust-lang.org/nomicon/atomics.html)

- These slides:
  * https://github.com/vorner/rust-presentations/blob/master/atomics.md
  * TODO: Publish the html version somewhere.

???

* Repeat what each one is for.
  - Relaxed: only the atomic is safe to modify from multiple threads, without
    any care for other memory.
  - Acq/Rel: For synchronizing other memory through a single atomic.
  - SeqCst: To also order operations on *multiple* atomics.
* Wish them happy hacking, but be careful.
* Invite to contributing :-)
