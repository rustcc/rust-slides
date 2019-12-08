# Build a VM in Rust

In this talk we will explain the basics of how to build a Virtual Machine, using
the Rust programming language; discuss the specifics of how we implemented
RISC-V in CKB-VM, and some of the problems encountered. Finally, we will use
CKB-VM to create a live networked video game called Ferris-Fencing that runs
arbitrary Rust code in a RISC-V sandbox.

Distribute flyers on community-swag table before the talk so people can
play with the game and gossip about it.

## Outline

- Pre-roll slide
  - Title
  - Link to repo / slides / code / live server instructions
- About us
- Session overview
  - Building a VM in Rust that also runs Rust
- Part 1: VM's
  - Why build a VM?
    - Generally, CKB, VMs are cool
    - Use cases
  - Something about ISAs?
  - RISC-V
    - Why RISC-V?
      - open ISA, minimal, simple, elegant
      - cool
    - RISC-V reference card visual
    - RISC-V industry adopters?
  - ?
- Part 2: Rust on a RISC-V VM
  - In this section
    - What's in a VM
    - Running Rust on a RISC-V VM
    - Writing a runtime for CKB-VM
  - Ferris Fencing redux
  - Ferris Fencing demo
  - CKB-VM
    - Anecdote: how I learned about CKB-VM
    - What's cool about CKB-VM
  - A VM-based architecture
    - A virtual machine (CKB-VM)
    - Client code (Ferris-bots)
    - A runtime (The Ferris Fencing "platform")
  - CKB's runtime interface
  - Making Rust `main` work on CKB-VM
  - Syscalls in CKB
- Contributing to CKB-VM and Ferris-Fencing
- Thanks
