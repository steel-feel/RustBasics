# Conversion

Primitive types can be converted to each other through casting.

Rust addresses conversion between custom types (i.e., struct and enum) by the use of traits. The generic conversions will use the From and Into traits. However there are more specific ones for the more common cases, in particular when converting to and from Strings.

## From

The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types.

```rust

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// .... Snipped

 let num = Number::from(30);

```

## Into

`into` is just vice-versa of `from`, It just looks for implementation of `from` into target type  

```rust
  let int = 5;
  
  let num: Number = int.into();

```

## TryFrom and TryInto

generic traits for converting between types. Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions

```rust 

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}


```



## String conversions
 
To convert any type to a _String_ , **One** should implement the `fmt::Display` trait which automagically provides `ToString` and also allows printing the type as discussed in the section on `print!`.

To parse a string to any other type such as number. Use `parse` function. There are two syntax approaches
        - Type inference 
        - _turbofish_ syntax

> This will work as long as `FromStr` trait is implemented for that type. Its built in for standard types.

```rust

  // type inference
  let parsed: i32 = "5".parse().unwrap();
  // turbo fish
  let turbo_parsed = "10".parse::<i32>().unwrap();

```