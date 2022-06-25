# Modules

Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

By default, modules has **private visibility**, could be overridden with the pub modifier to make it availabe to outside of the scope.

## Visibility variants

- declaring module `mod <moduel-name>`

- public : `pub fn my_function()`

- private : `fn my_priv_function()`

- Nested Modules:
```rust
    mod my_mod { 
    //..... snipped
    pub mod nested {

    }
    }
```  

- Visibility within given path ( Its like choosing the lexical scoping), path should be parent:

 ` pub(in crate::my_mod) fn public_function_in_my_mod() `

- Visibility within parent module : 

` pub(super) fn public_function_in_super_mod()`

- Visibility within crate
` pub(crate) fn public_function_in_crate()`

> Private parent items will still restrict the visibility of a child item, even if it is declared as visible within a bigger scope.

## Struct visibility
Structs have an extra level of visibility with their fields. Again, by default private, could be made public using `pub` . visibility only affects of accessed from outside the module.

> However, structs with private fields can be created using public constructors (functions discussed in previous section)

## Use

The use declaration can be used to bind a full path to a new name, for easier access. Similar to Javascript import, it also has `as` keyword to make an alias.

```rust
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

use deeply::nested::function as other_function;

fn main() {
    my_first_function();
    other_function();
}

```

## super and self

The super and self keywords can be used in the path to remove ambiguity when accessing items and to prevent unnecessary hardcoding of paths. This are similar to other languages. here self always refer called by module.

## File hierarchy
Modules can be mapped to a file/directory hierarchy.

`mod my;` This declaration will look for a file named `my.rs` or `my/mod.rs` and will
insert its contents inside a module named `my` under this scope.

Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs` and `inaccessible.rs` files and insert them here under their respective modules.
