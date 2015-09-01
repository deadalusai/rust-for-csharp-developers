# What is Rust?

A statically checked, compiled language intended as a modern
"systems" programming language.

It aims to provide zero-cost abstractions and memory safety -
using static (compile time) checking and deterministic memory
allocation/deallocation.


# Features

"Zero-cost" means that many code abstractions can be made "compile-time only".

The type inference engine can determine the type of a generic parameter based on
how the value is used later in a function - similar to languages like F# and Haskell.

The language does not have C/C#/Java-style "exceptions", though it does have the 
ability to "panic".

No nulls - Rust does not even have a "null" keyword.

No dangling pointers - a dangling pointer is a pointer into memory which has been freed.
Rust guarantees that no pointer will be left dangling.

No segfaults - "segmentation faults" or "access violations" - generally raised when a
program attempts to access memory to which it does not have access (e.g. via a dangling pointer).

All type-checking happens at compile time. Depending on compiler flags, even integer overflow
checks can be elided.

The rust compiler generates LLVM code, which is then optimized and compiled to machine code
by the excellent LLVM compiler backend.

Finally - Rust aims to be completely cross-platform. The recent 1.3 release has added support
for the MSVC compiler toolchain.

# Features continued

The ultimate goal is to produce a compiler which can statically prevent data races and
dangling pointers from ever occuring. 

So we can say that Rust...


# Ownership and Borrowing

Rust achieves this feat by introducing three key concepts: Ownership, Borrowing and Mutability

Ownership is the idea that every variable or piece of memory has a single owner.

Borrowing is the idea that ownership can be temporaily lent out - e.g. to a subroutine.


# Ownership

These code snippets demonstrate the concept of ownership. In the C# code above we
construct an instance of `Foo`, and then two instances of `Bar`. Each instance contains
a reference to `foo`.

Who owns `Foo`? In C# the answer is not critically important - the Garbage Collector
is there to clean up after us. In a language like C or C++ however it must be clear who
owns that data because it eventually needs to be freed!

The equivalent Rust code fails to compile. Instead it produces an error:

    `use of moved value: foo`


# Ownership two
    
The variable `foo` is "moved" into `bar1`, so bar1 now owns that data. We cannot
subsequently move it a second time.

In order to use that data elsewhere we must either copy it, access it via `bar1`
or share it through some other mechanism.


# Ownership three

Functions also consume ownership. When I invoke `subroutine` with the `foo` variable,
I give that function ownership of the variable.

I cannot subsequently use it - even if the function has completed and returned.


# Borrowing

Every borrow has a "lifetime". Lifetimes are generally defined by a local (lexical)
scope.

Here we borrow `foo` and assign the reference to a variable `temp`. Within the block
scope we can use `temp` as much as we like, but we are prevented from using foo.

We could rewrite the `subroutine` function to accept a reference - this would allow us
to invoke the subroutine without giving up ownership of our data.


# Borrowing two

Data once borrowed can only be accessed via that reference, until the reference goes
out of scope and the borrow ends.

This is to prevent us from "moving" (or otherwise changing) borrowed memory while the 
borrow is still in scope.

If you have a reference to some memory it is guaranteed to always be valid.


# Borrowing three

This common mistake in the C programming language is a compile error in Rust.

Here we return a pointer to a stack-allocated variable. GCC issues a warning:

    `function returns address of local variable`
    
But allows us to continue. Hello, use after free!

Rust fails to compile the equivalent code. The error message we're given:

    `missing lifetime specifier`
    
Means that Rust could not determine an appropriate lifetime for the borrowed
value, and so wants us to provide it.

We can't provide a lifetime, because it's impossible to do so. Use after free prevented!


# Borrowing four

We **can** return a reference into data which we've borrowed. Here the `get_inner` function
returns a reference to the `inner` field of the `Foo` struct.

Rust determines that `foo` and `inner` have the same lifetime, and so the borrow is allowed.

The rule is that `foo` must live **at least** as long as `inner`.


# Cloning

Sometimes you will find yourself "fighting with the borrow checker". A common way to resolve
this is to take ownership of a value you're borrowing.

You can use something like the `clone` function to take a copy of a value. Once you own the 
data, you can do what you like with it.

Keep in mind that cloning a large struct might be an expensive operation!




