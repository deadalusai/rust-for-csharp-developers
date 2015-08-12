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

1. Rich type-system
2. Powerful type inference
3. High-level, zero cost abstractions
4. No garbage collection! (But the compiler will free all your memory for you)
5. No exceptions! (Though code can panic)
6. No nulls! (There is no **null** keyword)
7. Entirely compile-time checking
8. Cross-platform (Support for MSVC toolchain just released)

---

# How?

Rust achieves its goal by encoding something new directly into the
language:

.center[**Ownership**]

This concept of "ownership" is core to the Rust language.

---

# Ownership 

Let's take a look at some simple C# code:

```csharp
class Foo { public int Data; }
class Bar { public Foo Foo; }

var foo = new Foo { Data = 1 };
var bar1 = new Bar { Foo = foo };
var bar2 = new Bar { Foo = foo };
```

Who owns `foo`?

The equivalent Rust code *does not compile:*

```rust
struct Foo { data: i32 };
struct Bar { foo: Foo };

let foo = Foo { data: 1 };
let bar1 = Bar { foo: foo }; 
let bar2 = Bar { foo: foo }; // error: use of moved value: `foo`
```

---

# What's going on here, then? 

```rust
let foo = Foo { data: 1 };
let bar1 = Bar { foo: foo };
```

Here, `foo` refers to a piece of memory - 32 bytes - allocated on the stack.

The second statement **moves** `foo` into `bar1`. Bar "consumes" ownership of the data - 
from this point forward the only way to access `foo` is through `bar`:

```rust
println!("The value of foo.data is: {}", bar1.foo.data);
```

The Rust compiler prevents us from using the `foo` variable again.

---

NOTES:
Move into the following...

- Ownership
- Borrowing
- Concurrency
- Unsafe

---
