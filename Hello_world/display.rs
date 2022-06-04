/*
fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance. 
This is done by manually implementing fmt::Display,
which uses the {} print marker. 

*/

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

//Assignment, Implement Display trait 
// for Complex Number struct having real, imag parts

struct Complex {
    real : i32,
    imag : i32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Real {}, Imaginary {}", self.real,self.imag)
    }
}


//Assignment 2, Pretty print custom type "List" of "Vec" with index and value
struct List(Vec<i32>);

impl fmt::Display for List{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec = &self.0;

        write!(f, "[")?;

        for (count,v) in vec.iter().enumerate() {

            if count != 0 {  
                 write!(f,", ")?;
            }
            write!(f,"{}:{}", count,v)?;
        }

        write!(f, "]")

    }


}


fn main() {
    //Assignment 1 test
    println!("{}", Complex{real: 3, imag :6});

    //Assignment 2 test
    println!("{}", List(vec!(32,76,19,43,89)));


}
