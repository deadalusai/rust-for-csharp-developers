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

class: center, middle

# Why should I be interested?

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

class: center, middle

# How does it do it?

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

The equivalent Rust code does not compile:

```rust
struct Foo { data: i32 } //i32 is a 32-bit signed integer
struct Bar { foo: Foo }

let foo = Foo { data: 1 };
let bar1 = Bar { foo: foo };
let bar2 = Bar { foo: foo }; // error: use of moved value: `foo`
```

.center[
[example](http://is.gd/r4a4fS)
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
[example](http://is.gd/u0caGa)
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
[example](http://is.gd/rOF3MS)
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
[example](http://is.gd/uNGBlG)
]

The lifetime of the borrow helps the Rust prevent **use after free**, by
forcing us to ensure that any referenced memory lives at least as long
as the borrowed values.

---

# Borrowing

**Borrowing** prevents **moving**.

```rust
let a = Foo;
let r = &a;
subroutine(a); // Error
```

I can't transfer ownership away until the borrow goes out
of scope.

```rust
let a = Foo;
{
    let r = &a;
}
subroutine(a); // Ok
```

.center[
[example](http://is.gd/ug1TDW)
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
[example](http://is.gd/aDm0KQ)
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
[example](http://is.gd/yfc1Bt)
]

---

# Borrowing (lifetimes)

The borrow checker can't always determine the lifetime of a borrow.

We can provide lifetime parameters to help describe the lifetime of
a value.

```rust
// In a struct
struct Bar<'a> {
    foo: &'a Foo;
}

// In a function
fn baz<'a>(foo: &'a Foo) {
    //...
}
```

The lifetime parameter helps the borrow checker verify that `Bar`
does not outlive the `Foo` reference it contains.

---

# Borrowing (giving up)

Fighting with the borrow checker?

```rust
let borrowed_foo: &Foo = ...;
let owned_foo: Foo = borrowed_foo.clone(); // An owned copy of borrowed_foo
```

Borrowing can become complicated (or impossible). A common solution is to
take a copy of a borrowed value.

Because you own the result, you can do what you like with it.

.center[
[example](http://is.gd/773lGD)
]

The `clone` function is part of the
[Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
trait.

---

# Memory management

Rust has no garbage collection - all memory management is deterministic.

```rust
fn main() {

    // `thing` is an owned, unique pointer to the heap-allocated MyBigStruct
    let thing = Box::new(MyBigStruct { });

    // thing goes out of scope here...
}
```

- We know the lifetime of `thing` (until the end of `main`)
- We know it is guaranteed to have one owner (us)
- So we know when it is safe to free the memory held to by `thing`

**Note:** The `box` keyword is not completely ready but is coming
soon:

```rust
let myBox = box 100_i32; // A boxed 32-bit integer
```

---

# Memory management

We can use the `std::ops::Drop` Trait to run code when a value
goes out of scope:

```rust
struct MyStruct { id: i32 }

impl std::ops::Drop for MyStruct {
    fn drop(&mut self) {
        println!("MyStruct {0} dropped!", self.id);
    }
}

fn main() {
    let first = MyStruct { id: 1 };
    {
        let second = MyStruct { id: 2 };
    }
    let third = MyStruct { id: 3 }
}
```

.center[
[example](http://is.gd/vu7uuW)
]

---

# Memory management

Sometimes you want to have shared memory. Reference counting is
another method for managing memory:

```rust
use std::rc::Rc;

fn main() {
    let thing = Rc::new(100_i32); // Reference count: 1
    {
        let thing2 = thing.clone(); // Reference count: 2
    } // Reference count: 1
} // Reference count: 0
```

Like `Box<T>`, `Rc<T>` stores its data on the heap, behind a
read-only pointer.

`clone` increments the reference count, while `drop` decrements it.

---

# Mutability

In Rust, values are **immutable** by default:

```rust
let thing = 100_i32;
thing = 500; // Not allowed
```

Mark a variable mutable with the `mut` keyword:

```rust
let mut thing = 100_i32;
thing = 500; // Ok
```

---

# Mutability

Mutability is also part of the **type** of a borrowed reference:

```rust
fn increment(i: &i32) {
    *i += 1; // Not allowed
}
```

```rust
fn increment(i: &mut i32) {
    *i += 1; // Ok
}
```

.center[
[example](http://is.gd/c0jvce)
]

The `*` sigil dereferences the pointer. Used on the left-hand-side of
an expression it allows us to modify the value at the end of the reference.

---

# Mutability

A value may be borrowed mutably **only once**. This prevents *aliasing*,
wherin we have two ways of accessing the same data.

```rust
let mut a = 100;
let first = &mut a;
// error: cannot borrow 'a' as mutable more than once at a time
let second = &mut a;
```

Allowing the first borrow to go out of scope frees us up to borrow mutably again:

```rust
let mut a = 100;
{
    let first = &mut a; // First mutable reference
}
let second = &mut a; // Second mutable reference
```

---

# Concurrency

Ownership and reference mutability allows threads to safely share memory:

```rust
fn main() {
    let mut peeps = HashMap::new();
    peeps.insert("John", "Cena");
    peeps.insert("Rhonda", "Rousey");

    // Arc is Atomically Reference Counted
    let arc1 = Arc::new(peeps);
    let arc2 = arc1.clone();

    // Ownership of each Arc is moved into the thread
    let first  = spawn(move || { println!("Rhonda {0}", arc1["Rhonda"]); });
    let second = spawn(move || { println!("John {0}",   arc2["John"]);   });

    //Wait for the threads...
    first.join().unwrap();
    second.join().unwrap();
}

```

.center[
[example](http://is.gd/xBGGCg)
]

---

class: center, middle

# A selection of language features

A brief look at some of the day-to-day syntax
and features you'll use when writing Rust code

---

# Iteration

In Rust iteration is handled exclusively by iteration, using the
`Iterator<T>` trait:

```rust
let a = vec![1, 2, 3];
for i in a.iter() {
    println!("The value is: {}", i);
}
```

Rust also provides a "range" syntax:

```rust
for i in 0..10 {
    //Iterate 0 through 9
}
```

The `Iterator` trait provides a "LINQ-like" library for filtering and
manipulating streams of data.

.center[
[the iterator trait](http://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
]

---

# Type Inference

Rust has very strong type inference (within a function).

```rust
// The vector is Rust's generic collection
let my_strings = Vec::new();

my_strings.push("Hello, world");
my_strings.push("Goodbye, sun");
```

Rust determines the generic parameter of `Vec<T>` based on the types
used with it later in the function.

I can provide a hint (or just part of a hint):

```rust
let my_things = (0..100).collect::<Vec<_>>();
```


---

# Structs

We've seen this a few times:

```rust
struct Vector3 { x: f64, y: f64, z: f64 }
```

You can define functions with an `impl` block:

```rust
impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }
    
    fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

fn main() {
    let v1 = Vector3::new(100.0, 200.0, 300.0);
    let v2 = v1.add(&v1);
}
```

.center[
[example](http://is.gd/bnHDGi)
]

---

# Tuples

Tuples provide a way to package up multiple values up anonymously;

```rust
let my_stuff = (100_i32, "Hello, world");

// Destructuring syntax
let (int, string) = my_stuff;
```

---

# Tuple Structs

You can combine tuple and struct syntax to produce a tuple struct:

```rust
struct Vector3(f32, f32, f32);
```

---

# Enums

Enums are structs on steroids. F# users might be more comfortable with the name
"Discriminated Unions".

```rust
enum MyEnum {
    A,
    B(i32),
    C { i: i32 }
}
```

I can use a `match` statement to get to the contents of an Enum:

```rust
fn main() {
    let maybe = MyEnum::B(100);
    match maybe {
        MyEnum::A       => println!("Got A"),
        MyEnum::B(i)    => println!("Got B: {}", i),
        MyEnum::C { i } => println!("Got C: {}", i)
    };
}
```

.center[
[example](http://is.gd/Lec2l8)
]

---

# Enums

Enums can also behave "traditionally":

```rust
enum State {
    Ready = 0,
    Steady = 1,
    Go = 2
}
```

**Note:** A value of a given enum type is the size of the largest
variant.

---

# Strings

Rust strings are UTF8 encoded byte arrays. Rust has two string
types (sort of):

+   `String` - A (potentially) mutable, growable UTF8 string.
+   `&str` - Called a "string slice" - an immutable pointer to a portion of
    a string plus a length.

A bare string literal has the type `&'static str` - that is, a static,
immutable string reference. You will often see code like the following
to convert a string literal into an owned, mutable string:

```rust
let my_string: String = "The String".to_owned();
```

---

# Expressions

In Rust almost all language constructs are Expressions (as opposed to
Statements).

```rust
let a: i32 = if true { 1 } else { 2 };

let b = {
    let c = "Hello, world";
    c.to_owned()
};
```

In any block, the last expression without a terminating semicolon
is the "result" of that expression.

A block without a "value" expression has the `Unit` type,
represented as `()`.

---

# Vec&lt;T&gt;

The basic generic collection in Rust is the Vec:

```rust
let mut a = Vec::new();

a.push("Hello, world".to_owned());
```

The Vec struct is a growable, heap-allocated collection.

---

# Slices

We can borrow any contiguous chunk of memory as a slice.

Like `&str`, a slice is an immutable pointer to a portion of memory
plus a length.

For example I can take a slice of a Vec:

```rust
let data: Vec<_> = make_big_vector();

// Slice syntax allows us to get an immutable slice
// of the contents of the vector

let subslice_1 = &data[10..];
let subslice_2 = &data[..200];
```

`subslice_1` has the type `&[Foo]`, read as "Foo slice".

.center[
[example](http://is.gd/rcQEnp)
]

---

# Traits

Superfically similar to C# interfaces. You can implement a trait for a type
as long as you control at least one of them.

```rust
// From the standard library
trait Clone {
    // &self: borrows the value
    // Self: Returns an owned value of the same type
    fn clone(&self) -> Self;
}

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z: self.z }
    }
}
```

Much of the standard library is built by composing together
small traits.

---

# Traits

Traits can also define static functions:

```rust
// A trait for describing types with a "default value" in libstd
trait Default {
    fn default() -> Self;
}

#[derive(Debug)]
struct MyThing(i32);

impl Default for MyThing {
    fn default() -> MyThing {
        MyThing(100)
    }
}

fn main() {
    let a: MyThing = Default::default();
    println!("Result: {:?}", a);
}
```

.center[
[example](http://is.gd/A6D7VB)
]


---

# Traits and Generics

Traits become more useful when used as bounds in generic types or functions:

```rust
fn clone_vec<T>(source: &Vec<T>) -> Vec<T> where T: Clone {
    let mut dest = Vec::new();
    for item in source.iter() {
        dest.push(item.clone());
    }
    dest
}

fn main() {
    let mut clonable = Vec::new();
    clonable.push(100_i32);
    let copy = clone_vec(&clonable); //Ok!

    let mut unclonable = Vec::new();
    unclonable.push(Unclonable);
    let copy = clone_vec(&unclonable); //Not allowed!
}
```

.center[
[example](http://is.gd/FCnCeR)
]

---

# Traits and Generics

From the standard library: [FromStr](http://doc.rust-lang.org/stable/std/primitive.str.html#method.parse)
and [parse](http://doc.rust-lang.org/stable/std/primitive.str.html#method.parse).

```rust
struct Foo(i32);

impl std::str::FromStr for Foo {
    type Err = (); // Cop out on error reporting
    fn from_str(s: &str) -> Result<Foo, ()> {
        //parse string as Foo...
    }
}

fn main() {
    let foo: Foo = "100".parse().unwrap();
    println!("Result: {:?}", foo);
}
```

.center[
[example](http://is.gd/XPgUiU)
]

The `parse` function is generic over `FromStr`. Rust looks
up a `FromStr` implementation for type Foo.

---

# Generics

Rust compiles generic functions into specialized versions tuned
for the generic type parameters.

```rust
trait Foo {
    fn bar(&self) -> i32;
}

fn example<T>(foo: T) where T: Foo {
    println!("The bar value: {}", foo.bar());
}
```

The compiler will emit a version of `example` for every type which
implements `Foo` (if necessary).

.center[
[example](http://is.gd/jDn5Th)
]

---

# Generics

Rust also supports dynamic dispatch through "Trait objects".

Passing a Trait by reference (rather than generic constraint) will
cause Rust to generate a vtable.

The compiler emits only a single version of the function which is
optimized for the vtable.

```rust
fn example_two(foo: &Foo) {
    println!("The bar value: {}", foo.bar());
}
```

In this case the Trait behaves much more like a C# interface.

.center[
[example](http://is.gd/sO5f1f)
]

---

# Macros!

Rust provides "hygenic macros" - these macros operate directly on the
AST at compile time, unlike C/C++ macros which are just string token
replacement before compilation.

Macros are often used to reduce repetitious syntax, e.g.

```rust
//make and initialize a vector of ints
let data: Vec<i32> = vec![1, 2, 3];
```

Which might expand to something like:

```rust
let data: Vec<i32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
```

---

# Unsafe

Rust is a memory safe language which executes in an unsafe environment.

Many abstractions in Rust are implemented using **unsafe** code.

We can disable many compiler checks by using the "unsafe" keyword:

```rust
let a = 3;

// "I know what I'm doing"
unsafe {
    // Raw pointers are only allowed in unsafe code
    let b = &a as *const i32 as *mut i32;
    *b = 4;
}

println!("{}", a); // Prints 4
```

Unsafe code is not discouraged, but it's usually unnecessary!

.center[
[example](http://is.gd/qSC7tr)
]

---

# Unsafe (foreign function interface)

All FFI code is unsafe.

```rust
#[link(name = "externallib")]
extern {
    fn MyExternalFunction() -> i32;
}

fn main() {
    let i = unsafe { MyExternalFunction() };

    println!("{}", i);
}
```

This is because the foreign code could potentially do anything
and the Rust compiler has no way of verifying it.

(Rust can bind to any language which supports C bindings)

---

# Unit Testing

Rust comes with a built-in unit testing framework.

Tests are written in-line in a module, next to the code they're testing.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn math_checks_out() {
    let result = add(100, 200);

    assert_eq!(300, result);
}
```

Test code is only compiled in "test" builds.

.center[
[example](http://is.gd/ZCWswE)
]

---

class: center, middle

# Demo


---

# References

I lifted content from these much better presentations:

*   [Intro to the Rust Programming Language](https://www.youtube.com/watch?v=agzf6ftEsLU) - Alex Crichton
*   [The Rust Programming Language](https://www.youtube.com/watch?v=d1uraoHM8Gg&feature=youtu.be) - Alex Crichton

Also be sure to check out:

*   The [Rust Website](https://www.rust-lang.org/)
*   The [Rust Book](https://doc.rust-lang.org/stable/book/)
*   The [Rust Standard Library Documentation](http://doc.rust-lang.org/stable/std/)