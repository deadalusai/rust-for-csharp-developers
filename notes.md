# What is Rust?

A statically checked, compiled language intended as a modern "systems"
programming language.

It aims to provide zero-cost abstractions and memory safety - using static
(compile time) checking and deterministic memory allocation/deallocation.


# Features

"Zero-cost" means that many code abstractions can be made "compile-time only".

The type inference engine can determine the type of a generic parameter based
on how the value is used later in a function - similar to languages like F#
and Haskell.

The language does not have C/C#/Java-style "exceptions", though it does have
the ability to "panic".

No nulls - Rust does not even have a "null" keyword.

No dangling pointers - a dangling pointer is a pointer into memory which has
been freed. Rust guarantees that no pointer will be left dangling.

No segfaults - "segmentation faults" or "access violations" - generally raised
when a program attempts to access memory to which it does not have access
(e.g. via a dangling pointer).

All type-checking happens at compile time. Depending on compiler flags, even
integer overflow checks can be elided.

The rust compiler generates LLVM code, which is then optimized and compiled to
machine code by the excellent LLVM compiler backend.

Finally - Rust aims to be completely cross-platform. The recent 1.3 release
has added support for the MSVC compiler toolchain.

# Features continued

The ultimate goal is to produce a compiler which can statically prevent data
races and dangling pointers from ever occuring.

So we can say that Rust...


# Ownership and Borrowing

Rust achieves this feat by introducing three key concepts: Ownership,
Borrowing and Mutability.

Ownership is the idea that every variable or piece of memory has a single
owner.

Borrowing is the idea that ownership can be temporaily lent out - e.g. to a
subroutine.


# Ownership

These code snippets demonstrate the concept of ownership. In the C# code above
we construct an instance of `Foo`, and then two instances of `Bar`. Each
instance contains a reference to `foo`.

Who owns `Foo`? In C# the answer is not critically important - the Garbage
Collector is there to clean up after us. In a language like C or C++ however
it must be clear who owns that data because it eventually needs to be freed!

The equivalent Rust code fails to compile. Instead it produces an error:

    `use of moved value: foo`


# Ownership two

The variable `foo` is "moved" into `bar1`, so bar1 now owns that data. We
cannot subsequently move it a second time.

In order to use that data elsewhere we must either copy it, access it via
`bar1` or share it through some other mechanism.


# Ownership three

Functions also consume ownership. When I invoke `subroutine` with the `foo`
variable, I give that function ownership of the variable.

I cannot subsequently use it - even if the function has completed and
returned.

This doesn't seem particularly useful - all we're doing is passing around
ownership. So...

# Borrowing

A variable can be *borrowed*. The borrower takes temporary ownership
of the variable.

Here we borrow `foo` and assign the **reference** to a variable `temp`.

Every borrow has a "lifetime". The lifetime is generally lexical - it
lasts until the scope in which it was made ends.

We could rewrite `subroutine` to accept a reference - this would allow us
to invoke the subroutine without giving up ownership of our data.


# Borrowing two

All borrowed values have a lifetime.

Data once borrowed can only be accessed via that reference, until the
borrow ends.

This is to prevent us from "moving" (or otherwise changing) borrowed memory
while the borrow is still in scope. This prevents a common C and C++ bug:
**use after free**.

If you have a reference to some memory it is guaranteed to always be valid.


# Borrowing three

In addition to preventing us from freeing borrowed memory, Rust prevents
us from moving "transferring ownership of" borrowed values.

The is again in service of preventing borrowed memory from becoming invalid.


# Borrowing four

This common mistake in the C programming language is a compile error in Rust.

Here we return a pointer to a stack-allocated variable. GCC issues a warning:

    `function returns address of local variable`

But allows us to continue. Hello, use after free!

Rust fails to compile the equivalent code. The error message we're given:

    `missing lifetime specifier`

Means that Rust could not determine an appropriate lifetime for the borrowed
value, and so wants us to provide it.

We can't provide a lifetime, because it's impossible to do so. Use after free
prevented!


# Borrowing five

We **can** return a reference into data which we've borrowed. Here the
`get_inner` function returns a reference to the `inner` field of the `Foo`
struct.

Rust determines that `foo` and `inner` can have the same lifetime, and so the
borrow is allowed.

The rule is that `foo` must live **at least** as long as `inner`.

# Borrowing six (lifetimes)

I mentioned earlier that all borrows have a lifetime.

The borrow checker can often determine the lifetime of a reference
automatically. When it can't, we can use "lifetime parameters" to
help it out.

Like generic parameters, lifetime parameters must be declared as part
of a type, trait or functionn.

Lifetime parameters are prefixed with a single apostrophe: `'`

# Borrowing seven (giving up)

You will not always want to work with borrowed values - either you want to
own the data you're working with, or you find yourself "fighting with the
borrow checker".

One way to take ownership of a borrowed value is to copy it - we can do so
with the `clone` function.

Once you own the data, you can do what you like with it.

Keep in mind that cloning a large struct might be an expensive operation!

`clone` is provided by the "Trait" `std::clone::Clone` - we'll talk more about
traits later.


# Memory management

Rust has no garbage collection - instead rust determines when it can free memory
by analyzing the lifetimes of variables and determining when they are safely out
of scope.

This is made possible because of ownership - every piece of memory at any time
can have exactly one owner.

Here we introduce a new concept - Boxing. `Box<T>` is the standard manner for
putting data on the heap. We pass some data to the `Box::new` function, which
allocates space on the heap and copies the data there.

We own the `Box<T>` instance, and the Box owns the pointer into the heap.

When the `thing` variable goes out of scope, we know it is safe to free the
memoryh eld by the Box.

**Note:** The `box` keyword is coming soon - it provides a language-integrated
way to allocate memory. Advantages include support for custom allocators and
avoiding allocating memory on the stack unnecessarily.


# Memory management two

We can use the `std::ops::Drop` Trait to run code when a value goes out of
scope.

This code will print in the order "2, 3, 1", as each value is dropped in the
reverse order in which it was declared in its scope.

There's a bunch of new syntax here. The `impl` block implements the `Drop` trait
for `MyStruct`. The rust compiler automatically inserts calls to `Drop::drop`
whenever each value goes out of scope.

We can use this function to clean up resources we own and free memory.


# Memory management three

You may want to decouple the lifetime of a value from the scope of your
function.

Box is one way to accomplish this, but it only allows a single owner.

If you want to have shared read-only memory, you can use an `Rc<T>`.

We can `clone` an Rc to hand out ownership to other structs and
functions. Every clone increments the reference count.

As each owned copy of the Rc is dropped, the reference count is
decremented. When it reaches zero, the memory is deallocated.


# Mutability

In Rust values are immutable by default. We can mark a variable
mutable using the `mut` keyword.


# Mutability two

Mutability is also part of the **type** of a borrowed reference.

In the first function we are passed an immutable 32-bit integer reference.
Attempting to write to this reference results in an error.

In the second function we are passed a mutable reference. Not only can I
read from it, but I can also write back to it too.


# Concurrency

Let's take a quick dip into concurrency.

Thanks to Ownership and Reference mutability, we can safely share memory
between threads.

`Arc` is "Atomically Reference Counted" - like Rc, but safe to use across thread
boundaries. It provides immutable access to the data it contains.

**Note** that you cannot share a **mutable** reference across threads!

The `spawn` function spawns another thread.

The `|| { }` syntax demarks a Rust closure. It's equivalent to the `() => { }`
syntax of C#.

The `move` keyword makes the closure "take ownership" of its environment.
All values used in the closer are moved **into** the closure.

The two Arcs are "dropped" when their threads exit.

**Note** that `HashMap` has no special handling for cross-thread usage - rust
ensures that the threads have only read-only access to `peeps`.


# More straightforward language featues

So far we've had an in-depth review of the features which make Rust new and
interesting.

Lets take a look at some of the more vanilla language features you'll use in
day-to-day rust.


# Structs

We've seen this syntax used throughout my samples so far.

Like C#, structs are a collection of values accessible by name.

Unlike C# a struct must be "constructed" in a single expression - no partially
initialized values allowed.


# Tuples

Tuples provide a way to package up multiple values up anonymously.

Similar to the `Tuple` struct in C#, but can be arbitrarily large and supports
special syntax for creating, destructuring and describing.


# Tuple Structs

You can combine tuple and struct syntax to produce a tuple struct.


# Enums

Enums are structs on steroids. F# users might be more comfortable with the name
"Discriminated Unions".

Each enum variant can optionally contain data. A value of a given enum type will
be the size of the largest variant.

We use the `match` statement to "unwrap" an enum. Like F#, the Rust compiler
ensures that this statement is exhaustive - if you fail to specify a branch
for an enum variant (or a catch-all branch) the compiler emits an error.


# Enums two

Enums can also behave more "traditionally".


# Strings

Rust strings are UTF8 encoded byte arrays. Rust provides us with functions
for enumerating bytes, characters or graphemes.

Rust has two types to represent strings: `String` - a mutable, growable
string type; and `&str` - called a string slice, a "fat pointer" into
a string.

A bare string literal has the type `&'static str` - a string slice with
a static lifetime. You will often see code calling `.to_owned()` to
copy the slice into an owned, mutable string.

Many, many functions in rust accept and return string slices, helping
to avoid expensive string copy operations.


# Expressions

In Rust almost all language constructs are Expressions (as opposed to
Statements).

In any block, the last expression without a terminating semicolon
is the "result" of that expression.

A block without a "value" expression has the `Unit` type,
represented as `()`.

For branching statements like `match` or `if`, the return type of each
branch must be the same.

Loops always resolve to the `Unit` type.

# Option<T>

Rust doesn't allow "null" references, but it can be useful to represent
the concept of "no data".

The standard library gives us the `Option<T>` enum. Option represents the
possibility of "Some value" or "None".

C#- or Java-style "null reference errors" are hard to cause accidentally - I
must explicitly "unwrap" an Option to produce Rust's equivalent.

Generally, you will use special Rust syntax to safely handle Option values.


# Result<T, Err>

Rust doesn't have any concept of "Exceptions". (Panics don't count*)

Instead we can use something like the standard library `Result` type.

This enum encodes success (Ok) and failure (Err). The values can
be accessed either through pattern matching or the `.unwrap()` function.

The standard library has lots of utility plumbing built around Result,
e.g. the `try!` macro.


# Traits

Traits are superfically similar to C# interfaces. You can implement
a trait on any type as long as you control the type or the trait
(or both).

In this case the "Clone" trait indicates that a value implementing
it can go from a shared reference "&self" to an owned value "Self".

We implement the trait using the "impl for" syntax.

This implementation copies the x, y, z values into a new Vector3
struct.


# Traits two

Traits can also define static functions. Here we've got a Trait defined
in the standard library: `Default`. It allows us to describe a type with
a "default" constructor or value.

Note the function does not take a "self" argument.

We can invoke this using the `Default::default()` syntax (the compiler
determines the correct method to call by inferring the return type from
the right-hand side.

We could also type `MyThing::default()` or `<MyThing as Default>::default()`.


# Traits and Generics

Traits become useful when used as bounds in generic types and functions.

Here we've got a generic function `clone_vec`, which accepts a vec of T
and attempts to clone its contents.

We can use `clone_vec` only with a vector which contains a type that implements
`Clone`. This is because of the `where T: Clone` bound.

The `Unclonable` struct here doesn't implement `Clone`, so it's not allowed.


# Traits and Generics two

Let's look at a cool example.

The `parse` function is generic over `FromStr`. Rust infers
the expected return type of parse and then looks up a FromStr
implementation for that type.

It is a compile error if an implementation cannot be found.

FromStr also defines an associated type: Err. The implementor
of FromStr must also describe the type returned in the event of
an error.


# Generics

Rust compiles generic functions into specialized versions tuned
for the generic type parameters through a process called
Monomorphization.

Here, the compiler may generate a multiple versions of `example`: one tuned for
each type which implements `Foo`.

These functions are invoked statically - no dynamic dispatch is required.

# Generics two

Rust also supports dynamic dispatch through "Trait objects".

Passing a Trait by reference (rather than generic constraint) will
cause Rust to generate a vtable for any type that implements that
Trait.

The compiler emits only a single version of the function which is
optimized for the vtable.

In this case the Trait behaves much more like a C# interface.


# Macros!

Rust provides "hygenic macros" - these macros operate directly on the
AST at compile time, unlike C/C++ macros which are just string token
replacement before compilation.

Rust code commonly makes extensive use of macros to reduce boilerplate
and automate common actions.

Macros are identified in code by the bangs (!) at the end of their name.
They allow for quite flexible syntax and include a small "type system",
which allows you to limit the types of tokes you accept as arguments.

You can write your own macros using the `macro_rules!` built-in macro.


# Unsafe

Rust is a memory safe language which executes in an unsafe world.

Rust provides you an interface with guarantees about memory safety,
but under the hood many operations are implemented using unsafe code.

This could be for performance reasons or simply because the is just not
smart enough to understand that a given operation is actually safe.

We can write unsafe code only with an unsafe block... C# developers
will be familiar with this concept.

This operation is unsafe becase the Rust compiler can't verify anything
done here is legitimate. Here I'm breaking the rules by mutating an
immutable variable.


# Unsafe (ffi)

All Rust code which interoperates with the outside workld is necessarily
unsafe.

All FFI (foreign function interface) code is unsafe. This is because the
foreign code could potentially do anything, including scribble on your stack.

For FFI, Rust basically speaks C - any language which supports binding to C
can bind to Rust code.