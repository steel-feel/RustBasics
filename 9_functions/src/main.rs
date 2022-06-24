#![allow(dead_code)]

#[derive(Debug)]
struct Pair {
    x: u32,
    y: u32,
}

//Diverging functions
fn foo() -> ! {

    panic!("I am not returning control");
    
}


impl Pair {
    // Associated function
    fn origin_point() -> Pair {
        Pair { x: 0, y: 0 }
    }

    //method
    fn destroy(self) {}

    //change cordinate
    fn change(&mut self, x: u32, y: u32) -> &Pair {
        self.x = x;
        self.y = y;
        self
    }
}

fn closures () {

    const CONSTANT:u32 = 10 ;

    let increment_by_const= |i:u32| -> u32 {  return i+CONSTANT; };

    let inferred_increment_by_const = |i| i + CONSTANT ;

    println!("{}", increment_by_const(1));
    println!("{}", inferred_increment_by_const(2));


}

fn main() {

    let mut new_pair = Pair { x: 0, y: 0 };

    println!("{:?}", Pair::origin_point());
    println!("{:?}", new_pair.change(2, 3));
    new_pair.destroy();
    //Will give error, as already destroyed
    // println!("{:?}", new_pair);
    //Diverging functions
    // foo();

    //Closures
    closures();
}

