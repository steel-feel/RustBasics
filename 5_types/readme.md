# Types

No implicit type conversions only explicit type casting allowed using `as` keyword. Literals can be type annotated using type as suffix. e.g. `7i16` .

default is i32 for integers, and f64 for floating-point numbers.

size of variable can by checked with a standard function

```rust
let a = 45i16;
std::mem::size_of_val(a)
```

## Inference

Type inference engine is smart :sunglasses: . It does more than looking at the type of the value expression during an initialization. It also looks at how the variable is used afterwards to infer its type.

> Note: Type can **only** be infered upto default values or generic types. e.g. if a number is too big to fit in default i32, a type has to be mentioned explicitly. check example in `main.rs` .

## Aliasing

The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning. 



```rust
type color = u8;
```