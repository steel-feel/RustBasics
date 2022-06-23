# Control flow

## if-else

Similar to any other language, condition doesn't need to surroinded by parentheses. Could use if-else to compute values.

```rust 


let num = 6;

    let is_even = if num%2 == 0 {
        true
    } else
    {
        false
    };


    println!("Is Number {} even ? : {} ", num, is_even);

```

## loop

Rust provides a loop keyword to indicate an infinite loop.The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration.

```rust 

loop {

if <some-condition> break;

}

```

In case of nested loops, we can have labels to deal with `break` and `continue`.

```rust

//----- Snipped -------
'outer : loop {

        'inner' : loop {

            if <some-condition> {
                //breaks inner loop
                break;
            }

            if <some-other-codition> {
            //break 'outer' loop
                break 'outer;
            }

        }


}

```

One can also return last value via `break` statement

```rust

let result = loop {

    if <some-condition> {
        break <return-value>;
    }

}

```

## while 

`while` is similar to any other languages.

## for loops

`for` construct could be used in two ways:

- `a..b` : a (inclusive) to b (exclusive) in steps of 1
    
- `a..=b` : a and b inclusive 

```rust


 for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

```

Usage with iterators
`for-in` construct is able to interact with an Iterator.

`into_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in different ways,

> by default the `for` loop will apply the `into_iter` function to the collection


- **iter** - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

```rust
 for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
```

- **into_iter** - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

```rust

 for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
//names no longer available, reduced into atoms 😀

```


- **iter_mut** - This mutably borrows each element of the collection, allowing for the collection to be modified in place. 

Its simliar to `map` of javascript;


```rust

 for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
}


```


> In the above types, note the type of match branch, that is the key difference in the types of iteration. The difference in type then of course implies differing actions that are able to be performed.

## match

`match` can be used like switch. match values could be a literal or a range (in case of numbers). 
`_` is used as default case.

> Match should cover all the cases, use `_` to catch all non-matching cases. 

```rust

  match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        
    }

```

`match` can be used against various data types .

- tuple :

```rust
let triple = (0, -2, 3);
  
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
```

- Arrays/slices : they are similar to tuples
- Enum : Already seen in _custom types_ .
- Pointers : For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts which are used differently from a language like C.

    - Dereferencing uses *
    - Destructuring uses &, ref, and ref mut
- Structs : Similar to above data types

```rust

 struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
    
```

### Guards
guard can be added to further filter the matching on destructing

```rust
fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

```

### Binding
Its is used to store the value for when doing a range matching.

```rust

fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```







