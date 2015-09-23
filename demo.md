# Demo

Demonstrate the use of the `Option<T>` and and `Result<T, E>` types.


## Demo 0 - Introducing `unwrap`

Let's look at a simple program which gets the first argument to our program, parses it
as an integer and prints it to the console.

The `nth` function returns an Option.

The `parse` function returns a Result.

Both expose an `unwrap` function which gives us access to the inner result.

But unwrapping comes at a cost - if the value is actually `None` (or `Error` in the case of Result)
then the unwrap funtion **panics**, terminating the thread.


## Demo 1 - Match your way to success

The next version is more "Rusty". We're using pattern matching against the results of these calls
which might fail to more correctly handle the error case.

Both the Option and Result types are **enums** - "algebraic" types whose members are part of a finite set.

We can safely get at their values using a `match` statement.


## Demo 2 - A more complicated example

Let's make a more complicated example.

This program reads from a file passed in on the command line.

We're using the "match" style error handling from the previous demo for the arguments, but I've
reverted back to `unwrap` for the next block.

Here we jump through some standard library hoops - Opening a file, creating a buffered reader, and
then enumerating the lines in the file and printing them out.

Each `unwrap` is a tiny code smell - what could go wrong with that function call? Whatever happens,
it'll crash our program.


## Demo 3 - Let's try matching again

Here's the same code sample but using pattern matching to handle the errors.

I've introduced a new function (`read_file`) whose return type is a Result of Vector of Strings.

We're handling all the error points now:

1.  If the file open fails, we report an error: "Could not open file"
2.  If the read fails while we're looping over the lines, we report an error: "An error occured..."
    (e.g. if the file was deleted while we were reading it)
    
If everything works out, we return the vector of Strings.

This code is super verbose! The program is now 50 lines long and doesn't do very much...


## Demo 4 - Functional style

The `main` function is unchanged, but read_file has been refactored into a more functional style.

Rather than matching on each result, we're using the functions built into the Result and Option
types to functionally reduce the values.

`map_err` invokes its closure if the Result is Err - we're ignoring the actual error value 
and returning a static string.

`map` invokes its closure if the Result is Ok - the file is passed to the closure, and we
invoke the next step in the process.

Create a BufReader.

Enumerate the Lines.

`.lines()` returns an Iterator or Results.

`.filter_map()` is a method on `Iterator<T>` - The closure we pass in converts the value `line` into
an option, and filter_map filters out any None values.

`.ok()` converts Result to Option, discarding any errors.

Finally, `collect` packages the results of the iteration into a collection. In this case Rust infers
that the collection should be a `Vec<String>`, based on the return type of the function.


## Demo 5 - Getting closer

`read_file` is now pretty tight, but it's doing something naughty - ignoring errors which occure during
the read process.

If the file is lost somehow before we collect the results, then read_file happily returns an empty vector.

Let's introduce something new: the `try!` macro.

This new version is very similar to our original "unwrap" code, but instead of unwrap, we're using 
the `try!` macro.

Whenever we need to unwrap a Result, we use the try! macro to do so.

See: `demo5_try.rs`.

The try! macro is defined something like this - the Ok branch resolves to the value, while the Err branch
resolves to an Early Return.

So when we try! to unwrap an Err, instead of panicking we simply pass that error back up to the calling code.