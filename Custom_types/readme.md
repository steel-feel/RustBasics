# Custom Types

Rust custom data types are formed mainly through the two keywords:

- struct: define a structure
- enum: define an enumeration

Constants can also be created via the const and static keywords.

## Structures

Destructure could be used to extract values from complex objects 

- Tuple struct

`
let Struct-Name(<new-varible> ,... ) = struct-instance
`
- C Type struct 

`
let Struct-Name{ <key> : <new-varible> , ...  } = struct-instance
`


## Enums

## Constants

### Quickies

To hide warnings for unused code, Use below attribute at top level
`
#![allow(dead_code)]
`

