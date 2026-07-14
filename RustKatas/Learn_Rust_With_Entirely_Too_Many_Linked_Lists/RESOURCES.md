# Rust Ownership (via Linked Lists) Resources

## Knowledge

- [Book: _Learn Rust With Entirely Too Many Linked Lists_ — aka "Too Many Lists"](https://rust-unofficial.github.io/too-many-lists/)
  **The primary source for this workspace.** `first.rs` = the "A Bad Singly-Linked
  Stack" chapter; `second.rs` = the "An Ok Singly-Linked Stack" chapter. Written
  specifically to teach ownership through building lists the hard way. Use for:
  the canonical explanation of every line we're studying.
  - first.rs chapter: https://rust-unofficial.github.io/too-many-lists/first.html
  - second.rs chapter: https://rust-unofficial.github.io/too-many-lists/second.html

- [Book: _The Rust Programming Language_ ("The Book") — official](https://doc.rust-lang.org/book/)
  The official, authoritative intro to Rust. Use for: foundational chapters when a
  concept needs more depth than the lists book gives.
  - Ch 4 Ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
  - Ch 6 Enums & pattern matching: https://doc.rust-lang.org/book/ch06-00-enums.html
  - Ch 10 Generics/Traits/Lifetimes: https://doc.rust-lang.org/book/ch10-00-generics.html

- [Std docs: `std::boxed::Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)
  Use for: the exact contract of `Box<T>` (heap allocation, ownership, drop).
- [Std docs: `std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
  Use for: `take`, `map`, `as_ref`, `as_mut`, `as_deref` — the methods `second.rs` leans on.
- [Std docs: `std::mem::replace`](https://doc.rust-lang.org/std/mem/fn.replace.html)
  Use for: the `first.rs` swap trick that `Option::take` later replaces.

## Wisdom (Communities)

- [r/rust](https://www.reddit.com/r/rust/)
  Large, welcoming subreddit. Use for: "is my mental model right?" questions once
  you can articulate them, and code review of your from-scratch rewrites.
- [The Rust Users Forum](https://users.rust-lang.org/)
  Higher-signal, beginner-friendly, official forum. Use for: careful questions about
  ownership/lifetime errors — post the exact compiler message.

## Gaps
- None yet. Add if a concept in `first.rs`/`second.rs` lacks a trusted explainer.
