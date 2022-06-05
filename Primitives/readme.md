# Primitives

## Scalar Types
- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- and the unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

## Points to remember


- **Integers** default to i32 and **floats** to f64

- Type can be annotated by suffix like _12i8_
- The type of a variable can't be changed. even for mutable but could be overshadow by re-declearing it. 
- Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.
- Underscores can be inserted in numeric literals to improve readability

## Arrays and Slices
Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; length].

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture

Slices can be used to borrow a section of an array, and have the type signature &[T].