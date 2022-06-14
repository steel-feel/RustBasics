# Custom Types

Rust custom data types are formed mainly through the two keywords:

- struct: define a structure
- enum: define an enumeration

Constants can also be created via the const and static keywords.

## Structures

Destructure could be used to extract values from complex objects 

- Tuple struct

```
let Struct-Name(<new-varible> ,... ) = struct-instance
```
- C Type struct 

```
let Struct-Name{ <key> : <new-varible> , ...  } = struct-instance
```


## Enums
The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid as an enum

Enum could be 
- unit-like
- tuple struct
- c-type struct

```
enum <enum-name> {
    <values>
}
```

Could be used with `match` which is similar to `switch` of other languages.

```
match <enum-variable> {
    <values> => {
        <statements-block-scope>
        },
    .......    
}
```
### use
The `use` declaration can be used so manual scoping isn't needed for values in enum

```
use <enum-Name>::<value>

//Value could be used without <enum-name::>
```

### Implicit discriminator

Enum values has implicit discriminator (starts at 0) (similar to solidity enum with is uint8). could be assigned manual values 
```
. . . .
value = <literal-value>

```

## Constants
- **const**: An unchangeable value (the common case).
- **static**: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.



### Quickies

To hide warnings for unused code, Use below attribute at top level

```
#![allow(dead_code)]
```

