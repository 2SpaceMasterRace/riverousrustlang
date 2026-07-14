# Teaching Notes

## Learner profile
- **Rust is their first language.** No prior programming background. Build stack/heap,
  pointers, allocation, types from zero. Never assume jargon from C/Python/etc.
- Has already *typed out* `first.rs` and `second.rs` following the book (git history
  confirms: Drop, Iter, IterMut, IntoIter, peek all implemented). So they have
  **exposure/fluency but not storage strength** — the classic illusion of mastery.
  Teach for genuine understanding + ability to reproduce from memory.

## How they want to be taught
- "Explain each and every line" — what it denotes/signifies, how it works under the
  hood (compiler + memory). They value the *why* and the mechanics, not just the *what*.
- Goal is reproduction from a blank file. Lean on retrieval practice and "build it blind"
  challenges, not copy-along.

## Course arc (planned, will adjust)
1. **[done]** The shape of a list: struct, enum, the recursion problem, Box. (first.rs 1-16)
2. Construction & ownership moves: `new`, `push`, `mem::replace`, moves vs copies. (first.rs 18-30)
3. `pop`, pattern matching with `match`. (first.rs 32-40)
4. `Drop`, `while let`, why the naive recursive drop blows the stack. (first.rs 49-60)
5. second.rs refactor 1: `Option` replaces the custom enum; `take`, `map`.
6. Generics: `List<T>`, `Node<T>`, why `<T>` everywhere.
7. References & `peek`: `&`, `&mut`, `as_ref`, `as_mut`.
8. Lifetimes: `Iter<'a, T>`, what `'a` means and why the compiler needs it.
9. The three iterators: IntoIter, Iter, IterMut; `as_deref`/`as_deref_mut`.

## Notes to self
- The files define local `trait Drop` and `trait Iterator` shadowing std — the book
  does NOT do this; it uses the real std traits. Flag this when we reach those lessons;
  it may cause confusion later. (Not urgent for lessons 1-3.)
