/// Macro to generate math operation functions
macro_rules! greetings {
    ($math_op:ident) => {
        fn $math_op(x: i32, y: i32) -> i32 {
            let a = stringify!($math_op);
            match a {
                "add" => {
                    return x + y;
                }
                "sub" => {
                    return x - y;
                }
                "div" => {
                    return x / y;
                }
                "mul" => {
                    return x * y;
                },
                _ => {
                  panic!("invalid operation");
                }
            }
        }
    };
}


greetings!(add);
greetings!(mul);
greetings!(jaadu);


fn main() {
   let x=10;
   let y=2;
    println!("addition result {}", add(x, y));
    println!("Multiply result {}", mul(x, y));

    println!("Jaadu operation {}", jaadu(x, y));
}
