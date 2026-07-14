# Mission: Master Rust's Ownership Model (via linked lists)

## Why
Rust is my first programming language, and ownership/borrowing/lifetimes are the
thing that makes Rust *Rust*. I'm using the "Too Many Linked Lists" book as the
vehicle: linked lists force you to confront ownership head-on. I've already typed
out `first.rs` and `second.rs`, but I don't truly understand them. I want to
internalize every line — what it means, and what the compiler and the machine's
memory actually do — until I can rebuild both files from an empty editor.

## Success looks like
- I can write `first.rs` from scratch, from memory, and explain *why* each line is there.
- I can write `second.rs` from scratch, from memory, including generics, `Option`, closures, and lifetimes.
- Given any ownership error the compiler throws, I can predict it before I hit compile, and explain the fix.
- I can draw the memory diagram (stack + heap) for a list at any point during push/pop.

## Constraints
- Rust is my first language — no prior programming background to lean on. Concepts
  like stack vs heap, pointers, and allocation must be built from zero.
- I learn best by understanding *why*, then proving it by reproducing code from memory.

## Out of scope (for now)
- The later chapters (`third.rs` onward: Rc, RefCell, unsafe, arenas). We finish
  mastering `first.rs` and `second.rs` first.
- Async, macros, and the wider Rust ecosystem (cargo internals, crates).
