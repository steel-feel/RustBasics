# Traits

A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.

## Derive
We can provide basic implementations using `#derive` attribute. they can also be manually implemented. 
Already seen `Debug` trait which lets print any arbitary object. 

## Returning Traits
If not sure about the returning the concrete type, we can however return trait implemented by the type using keyword `dyn` .

```rust
// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
```

## Operator overloading
Operators like +, -, can be overloaded meaning, there meaning can be changes w.r.t. to implemented type.

> Check `operator_overloading` example

## Drop
The Drop trait only has one method: drop, which is called automatically when an object goes out of scope. The main use of the Drop trait is to free the resources that the implementor instance owns.

## `impl` Trait
can be used in two locations:

### as an argument type
If dont mind as generic type as long as implement a trait use `impl` with argument

```rust

fn parse_csv_document(src: impl std::io::BufRead) -

```

- as a return type
Similar to argument, we can sepecify to return a type which implements certain trait. 

Very useful when need to return a closure

```rust
// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

## Supertrait
Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.

```rust
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

```

