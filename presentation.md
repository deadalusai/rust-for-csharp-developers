class: center, middle

# The Rust Programming Language
## (for C# developers)

Benjamin Fox | [Intergen](http://teamintergen.com/)

---

# What is Rust?

[http://rust-lang.org/](http://rust-lang.org/)

A statically checked, compiled language intended as a modern
"systems" programming language.

It aims to provide zero-cost abstractions and memory safety -
using static (compile time) checking and deterministic memory
allocation/deallocation.

---

# Why should I care?

1. Entirely compile-time checking
2. Cross-platform (Support for linking with MSVC just released)
3. High-level, zero cost abstractions
4. No garbage collection

**NOTES:**
Cross-platform: rustc 1.2 includes support for linking using the 
msvc toolchain.

"Zero cost abstractions" means that when written correctly, most
abstractions can be optimized away to static function calls.

---

# Anything else?

No exceptions! (Though code can panic)

No nulls! (There is no **null** keyword)

No GC! (But the compiler will free all your memory for you)

---

# How?

Rust achieves this feat by encoding two things directly into the
language:

- Ownership, and
- Borrowing



Rust guarantees memory safety (without GC), so you must be explicit
about who "owns" any given piece of memory.

This concept of "ownership" is core to the Rust language.

NOTES:
Move into the following...

- Ownership
- Borrowing
- Concurrency
- Unsafe

---
