# Generics
Generics is the topic of generalizing types and functionalities to broader cases. The simplest and most common use of generics is for type parameters.

```rust


fn foo<T>(arg: T) { ... };

struct bar<T>{T};

```


`<T>` is necessary as it tells compilier to not look for types that are in args, and consider them as Generics.

We specify what kind of data structure support using below syntax it is called **bounding**. bounding lets generic instances to access the methods of traits specified in the bounds. For example:

`<T:{trait to look for}>`

## Multiple bounds

Multiple bounds for a single type can be applied with a `+`. Like normal, different types are separated with `,`.

```rust
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}
```

`where` could also be used found bounding 

```rust
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

## Phantom type parameters

A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. These extra parameters hold no storage values, and have no runtime behavior.

