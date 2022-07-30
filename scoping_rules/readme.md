# Scoping rules

Rules which identifies ownership, borrowing and lifetimes.

rust enforces **RAII** (**R**esource **A**cquisition **I**s **I**nitialization), whenever a varibale goes out of scope its freed.


## Destructor

Implemented using `Drop` trait. Its kind of a hook which is called when resource goes out of scope.

## Ownership

A Resource can only have one owner at a time. When a _heap_ allocated item is assigned its ownership is **transferred**, 
even if the values is passed to function, in that case function argument is the owner for then on. It will get destroyed when the execution of function ends.

however for _stack_ allocated its **copied**. 

Mutability of data can be changed when ownership is transferred.

### Partial moves
In a struct, its possible to `move` few elements, `borrow` other elements.

> check `partial_borrow()` in _main.rs_

## Borrowing
If dont want to take the ownership we borrow the resource using `&` (while giving) and keyword `ref` (while taking).

mutable objects can be borrowed the similar way using `&mut` and `ref mut`. But only one mutable reference can exist in given scope. The original data can be borrowed again only after the mutable reference has been used for the last time.

the `ref` keyword can be used to take references to the fields of a struct/tuple while destructuring.

> check `borrowing()` in _main.rs_


## Lifetimes
A _lifetime_ is a concept for compiler to ensure the borrows are valid. life time begin and end with instantiation and destruction. usaually _lifetime_ and _scope_ are reffered in same way, but they are not **always**.


### Functions

Ignoring elision, function signatures with lifetimes have a few constraints:

any reference must have an annotated lifetime.
any reference being returned must have the same lifetime as an input or be static.
Additionally, note that returning references without input is banned if it would result in returning references to invalid data. The following example shows off some valid forms of functions with lifetimes:

```rust
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}


fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

```

### Static
Rust has a two reserved lifetime names.
1. constant with `static` declaration
2. Make a string literal which has type: &'static str.


### Elision
Some lifetime patterns are overwhelmingly common and so the borrow checker will allow you to omit them to save typing and to improve readability. This is known as **elision**.

every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references.The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.

The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.


- Compiler assigns a lifetime parameter to each parameter that’s a reference.

- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

- If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 

