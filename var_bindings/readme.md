# Variable bindings

Compiler is able to infer the type of variable from the context. But we can annotate with type as well.
Compiler will warn about unused variable bindings; these warnings can be silenced by prefixing the variable name with an underscore.

It's possible to declare variable bindings first, and initialize them later. The compiler thorws error if there are uninitialized variables, as this would lead to undefined behavior.

## Scope
Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}

## Mutability
Variable bindings are immutable by default, but this can be overridden using the mut modifier.

```rust
let mut a_string = "I can change";
```

Compilier strictly follows immutability even under seperate scope rules with shadowing.

