# macro_rules!
allows metaprogramming. looks like functions, but it ends with a `!` . gets expanded when called.

## Syntax
arguments are prefixed by `$`



```rust

//macro_rules! is keyword to define macro
//create_function is identifier
macro_rules! create_function {
    // ident is a type, for taking name of a function/variable
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

```

## Overloading
Macros can be overloaded, just like function.Arguments can be seperated by logicial or ',' .

## Multiple arguments
Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.

## Domain Specific Languages (DSLs)
ToDo