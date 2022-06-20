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

`for` construct could be used in two ways
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