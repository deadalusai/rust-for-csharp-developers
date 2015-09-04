class: center, middle

# The Rust Programming Language
## (for C# developers)

Benjamin Fox | [Intergen](http://teamintergen.com/)

---

# What is Rust?

http://rust-lang.org/

A statically checked, compiled language intended as a modern
"systems" programming language.

It aims to provide zero-cost abstractions and memory safety -
using static (compile time) checking and deterministic memory
allocation/deallocation.

---

# Features

01. Rich type-system
02. High-level, zero cost abstractions
03. Powerful type inference
04. No garbage collection!
05. No exceptions!
06. No null pointers!
07. No dangling pointers!
08. No segfaults!
09. Entirely statically checked
10. Cross-platform (Support for MSVC toolchain just released)

---

## Prevents almost all crashes

...by preventing segfaults/invalid memory access


## Eliminates data races

...by preventing two threads of execution from accessing the
same memory simultaneously

---

Rust introduces three key concepts:

.center[
# Ownership, Borrowing + Mutability
]

**Ownership** - every variable has a single "owner".

**Borrowing** - a variable can be "borrowed" temporarily.

**Mutability** - a variable can be marked immutable or mutable.

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

The equivalent Rust code *does not compile:*

```rust
struct Foo { data: i32 } //i32 is a 32-bit signed integer
struct Bar { foo: Foo }

let foo = Foo { data: 1 };
let bar1 = Bar { foo: foo }; 
let bar2 = Bar { foo: foo }; // error: use of moved value: `foo`
```

.center[
[playpen](http://is.gd/r4a4fS)
]

---

# Ownership 

```rust
let foo = Foo { data: 1 };
let bar1 = Bar { foo: foo };
```

The second statement **moves** the data owned by `foo` into `bar1`.

Bar "consumes" ownership of the data - from this point forward the
only way to access `foo` is through `bar`:

```rust
println!("The value of foo.data is: {}", bar1.foo.data);
```

The Rust compiler prevents us from using the `foo` variable again.

---

# Ownership 

Ownership extends to functions and subroutines.

```rust
struct Foo;

fn subroutine(foo: Foo) {
    //...
}

fn main() {
    let foo = Foo;
    subroutine(foo);
    subroutine(foo); // error: use of moved value: `foo`
}
```

Invoking a function with an argument "moves" ownership of that
argument into the function.

.center[
[playpen](http://is.gd/u0caGa)
]

---

# Borrowing

A variable or value can be *borrowed* for a period of time. 

```rust
fn subroutine(data: &Foo) {
    //...
}

fn main() {
    let foo = Foo;
    subroutine(&foo);
    subroutine(&foo); // Ok! Ownership was not consumed
}
```

The ampersand (`&`) sigil indicates a "reference". We can loan
out references to data we own as many times as we like.

.center[
[playpen](http://is.gd/rOF3MS)
]

---

# Borrowing

Every borrow has a "lifetime".

```rust
let foo = Foo;
{
    let temp = &foo;
    //subroutine(foo); // Not allowed
}
subroutine(foo); // Ok
```

As soon as the borrowed variable `temp` goes out of scope we can
again use the `foo` variable.

.center[
[playpen](http://is.gd/uNGBlG);
]

---

# Borrowing

**Borrowing** prevents **moving**.

```rust
let a = Foo;
let r = &a;
subroutine(a); // Error
```

I can't transfer ownership of `a` away until the borrow goes out
of scope.

```rust
let a = Foo;
{
    let r = &a;
}
subroutine(a); // Ok
```

.center[
[playpen](http://is.gd/ug1TDW)
]

---

# Borrowing

A common mistake in C:

```C
int* subroutine() {
    int i = 100;
    return &i;
}

void main() {
    int* intptr = subroutine();
}
``` 

The equivalent Rust code fails to compile:

```rust
fn subroutine() -> &int { 
    let i = 100;
    return &i;  // Error
}

fn main() {
    let int_ptr = subroutine();
}
```

.center[
[playpen](http://is.gd/aDm0KQ)
]

---

# Borrowing

We can return a reference to something we borrowed, though...

```rust
struct Foo { inner: i32 }

fn get_inner(foo: &Foo) -> &i32 {
    return &foo.inner;
}

fn main() {
    let foo = Foo { inner: 1 };
    let inner = get_inner(&foo); // `inner` has the same lifetime as `foo` 
}
```

Rust automatically determines that the `foo` parameter will live
at least as long as the borrow of `foo.inner`.

.center[
[playpen](http://is.gd/yfc1Bt)
]

---

# Cloning

Fighting with the Borrow Checker?

```rust
let borrowed_foo: &Foo = ...;
let owned_foo = borrowed_foo.clone(); // An owned copy of borrowed_foo
```

Borrowing can get complicated quickly. A common out is to take a
copy of a borrowed value.

Because you own the result, you can do what you like with it.

.center[
[playpen](http://is.gd/773lGD)
]

**Note:** The `clone` function is part of the 
[Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
trait - we'll cover that later!

---


