title: Lockfree data structures without a GC
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

* Lock-free algorithm/data structure.
  - What is it?
  - What are the advantages?
* What problems do we get without a GC.
  - Languages like C, C++, Rust.
* Why even bother?
  - Good mental exercise.
  - Someone has to implement the libraries.
  - High-performance code.
  - Use mutex by default 😇.

---

# A lock-free data structure

* Lock-free data structures don't use locks.
  - Nevertheless can be used from multiple threads.
* Resilience: they guarantee *eventual progress*:
  - A thread can't suspend or crash while keeping a lock.
* Performance:
  - Usually better during contention.
  - Very short critical sections.
* Thread *tries* to update:
  - Atomic CaS operation.
  - Retries the attempt if changed in between.
* Pose interesting problems and difficulties.

---

# Example: Treiber's stack, push

* It's a singly-linked list.
* Top is the head.

1. Read the head pointer.
2. Prepare the new node, linking to the head.
3. *Try* updating the head with the new node.
  - Success if it's still the same.
  - Failure if changed by other → update new node & retry.

---

# Example: Treiber's stack, push

```rust
fn push(stack: &Stack, value: usize) {
    let head = stack.head.load(Ordering::Acquire);
    let mut node = Box::new(Node { value, next: head });
    loop {
        let result = stack.head
            .compare_and_swap(node.head, &node, Ordering::AcqRel);
        match result {
            // Disable destructor and leave
            Ok(_) => break Box::leak(node),
            Err(new_head) => node.next = new_head,
        }
    }
}
```

---

# Example: Treiber's stack, pop

* The reverse of `push`.

1. Read the head.
2. Try to update head with `next`.
  - Retry from 1. on failure.
3. Get rid of the node.

--
  - **Ooops!!**
  - Another thread may be using it (between 1. and 2.).
  - But someone has to free it.
  - GC would solve it for us, but *we don't have it ☹*.

---

# The Wrong Solution

* Wait for other threads to *stop* using it.
* Not obvious how.
* Now we lose the lock-free guarantees.
  - Another thread not progressing blocks us.

---

# Generation-based reclamation strategy

* Postpone freeing once nobody can be using it.
  - Err on the safe side.

1. Remove the node from the data structure.
2. Stash it for later.
  - And make a note which threads are *active*.
3. Do something else in between.
4. Once all the active threads make enough progress, free it.
  - They stop using any pointers into the data structure.

---

# Generation-based reclamation strategy

.left-column[
* Keep a global generation number around.
* Pin a generation before accessing the DS.
* Unpin after having done everything.

- Old and new generation.
- If old equals new, moves new.
- Eventually everyone migrates.

* Older than old can be freed.
  - Otherwise we wouldn't have advanced the new.
]
.right-column[
![Generations](lockfree-non-gc/generations.svg)
]

---

# Properties

* Low CPU overhead.
  - Just pinning the generation.
  - Amortized over all operations in one pin.
  - Doesn't walk live data.
* Can delay reclamation.
  - Not good for RAII resources.
* Stuck thread can prevent all reclamation.
  - No guaranteed limit on memory overhead.
  - Small enough in practice.

???

* Crossbeam claims pinning is 3 atomic operations + barrier

---

# In practice

* [crossbeam](https://docs.rs/crossbeam-epoch/).
  - [deque](https://docs.rs/crossbeam-deque/),
    [MPMS channel](https://docs.rs/crossbeam-channel).
  - [contrie](https://docs.rs/contrie).
* [ssmem](https://github.com/LPD-EPFL/ssmem).
* Quescient State-Based Reclamation.
  - Similar principle, reports *silent* times instead of critical sections.
  - [liburcu](http://liburcu.com).
  - [Junction](https://github.com/preshing/junction).

???

* deque ‒ for worker executors, tokio based on that
* ssmem ‒ similar to crossbeam, for C

---

# Hazard pointers

* Global registry of pointers in use.
  - „Don't you dare free my pointer“.
  - Registry used for all pointers currently in use.
  - Can be used to pass an obligation to free once done with it.

---

# Hazard pointers, using the DS

1. Read the pointer.
2. Put it into registry.
3. Read again to verify, rollback registry if changed.
4. Use it.
5. Remove from registry.
6. If tagged with freeing obligation, proceed to removing.

---

# Hazard pointers, removing

1. Remove from the DS.
  - So nobody can get a *fresh* copy any more.
  - All pointers in use are already in the registry.
2. Check the *whole* registry.
3. Not found → free.
3. Found.
  - Delegate the freeing obligation to it.
  - „You're still using it, so clean up after yourself when you're done.“

---

# Properties

* Overhead per used pointer.
* Relatively more expensive freeing.
  - Can push the expense to random other thread.
* Implementation difficulties:
  - Extending the registry.
  - Shrinking the registry.
* Freeing not delayed.
  - Limited memory overhead (only the registry).
  - RAII-friendly.

???

* The registry is usually a linked list.
  - The same problems as we are solving.
  - We probably don't want to recurse or use the generations, further overhead.

---

# In practice

* [Concurrency building blocks](http://amino-cbbs.sourceforge.net/) (C, C++,
  Java).
* [Concurrency kit](http://concurrencykit.org/) (C).
* [Atomic Ptr Plus](http://atomic-ptr-plus.sourceforge.net/) (C++).
* [libcds](http://libcds.sourceforge.net/) (C++).
* [HazardCell](https://github.com/stjepang/atomic/blob/master/src/hazard.rs)
  (Rust).
  - Not yet finished/released.

---

# Arc-swap

* Hybrid approach
  - Limited hazard-pointer registry (no growing/shrinking).
  - Fallback to generation-inspired spin lock.
* Guaranteed wait-free reads.
  - Except the first one in each thread.
* Lock-free writes under specific conditions.
  - Not too many hazards in the current thread at once.
* https://docs.rs/arc-swap.

---

# Conclusion

* Other things exist too.
  - [Differential reference counting](http://www.1024cores.net/home/lock-free-algorithms/object-life-time-management/differential-reference-counting).
* Harder than with a GC.
* Can be solved.
  - With competitive performance.
* Further sources:
  - [Crossbeam's wiki](https://github.com/crossbeam-rs/rfcs/wiki).
  - [Introduction of Crossbeam](https://aturon.github.io/blog/2015/08/27/epoch/).
  - [Paper about generations](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf).
  - [Paper about hazard pointers](https://www.research.ibm.com/people/m/michael/ieeetpds-2004.pdf).
* Link to the presentation:
  - https://github.com/vorner/rust-presentations/lockfree-non-gc.md
