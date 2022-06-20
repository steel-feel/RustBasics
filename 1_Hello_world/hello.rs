fn main() {
    // Statements here are executed when the compiled binary is called
    let flight = 21;
    // Print text to the console
    println!("Hello World!");

    //In line comments
    /*
        Block comments 
     */

    
   //~~~~ Formatted output ~~~~~
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);


    //Positional arguments, Zero based

    println!("World largest countries {0}, {1}, {2}", "Russia", "China", "Brazil");

    //For custom types string print will not work

    //Rust 1.58 above can capture variables around it
  
    println!("{flight}");

    /*
    derive trait Debug to make it "print"able using debug ({:?})
    All types can derive (automatically create) the fmt::Debug implementation.
    */
    println!("{:#?}",DebugPrintable(85) );

    //

}

#[derive(Debug)]
struct DebugPrintable(i32);

