# Functions
Functions are declared using the fn keyword. Its arguments are type annotated, just like variables, and, if the function returns a value, the return type must be specified after an arrow ->.

The final expression in the function will be used as return value. Alternatively, the return statement can be used to return a value earlier from within the function, even from inside loops or if statements.

## Associated functions 
These functions attached to and accessed by struct **NOT** instance. These functions are generally used like constructors. Used using `::` syntax. They can **NOT** be used using _.(dot)_ syntax. 
```rust
//example Pair is struct and fixed_point is Associated functions
Pair::fixed_point()
```
## Methods
Methods are also functions but they have reference for self object `&self` as **first** argument. this reference could be mutable too or object itself(i.e. without &self). the first argument `&self` is implicitly passed while calling i.e. no need to pass argument. 

```rust,editable
//~~~~~~~~~~~~~ Snipped ~~~~~~~~~~~~~~~
    fn change(&mut self, x: u32, y: u32) -> &Pair {
        self.x = x;
        self.y = y;
        self
    }

//~~~~~~~~~~~~ Snipped ~~~~~~~~~~~~~~~~~~

 new_pair.change(2, 3)
```


## Closures

Its similar to javascript, `fn` cant use any variable in outer function(i.e. lexical scope), use closures. its anonymous, use for binding them to references. 

```rust
let c_annotated = |i:32| -> i32 {   i + 1   };
//OR
let c_inferred = |i:32|  i + 1  ;
```

## Diverging functions
Diverging functions never return. They are marked using !, which is an empty type. best example of it would be `panic!`

Although this might seem like an abstract concept, it is in fact very useful and often handy. The main advantage of this type is that it can be cast to any other one and therefore used at places where an exact type is required, for instance in match branches. 

```rust

fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));

```

It is also the return type of functions that loop forever (e.g. loop {}) like network servers or functions that terminate the process (e.g. exit()).

