# Demo

Demonstrate the use of the `Option<T>` and and `Result<T, E>` types.

## Introducing `unwrap`

Let's look at a simple program which gets the first argument to our program, parses it
as an integer and prints it to the console.

The `nth` function returns an Option.

The `parse` function returns a Result.

Both expose an `unwrap` function which gives us access to the inner result.

See `main.rs`

But unwrapping comes at a cost - if the value is actually `None` (or `Error` in the case of Result)
then the unwrap funtion **panics**, terminating the thread.

## Match your way to success

Both the Option and Result types are **enums** - "algebraic" types whose members are part of a finite set.

```rust
enum Option<T> {
    Some(T), None
}

enum Error<T, E> {
    Ok(T), Err(E)
}
```

We can safely unwrap their values using a `match` statement:

See `demo1.rs`

