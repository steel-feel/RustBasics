#![allow(dead_code)]

#[derive(Debug)]
enum Color {
    Blue,
    Black,
    Red
}

#[derive(Debug)]
struct apparel {
    cloth_type: String,
    wear_location: String,
    color: Color
}

fn partial_borrow(){
    let t_shirt = apparel { cloth_type :  "cotton".to_owned(), wear_location: "bottom".to_owned(), 
    color: Color::Blue };

let apparel { cloth_type: my_cloth_type, wear_location : ref my_wear_location, ..}  = 
t_shirt;

println!("my cloth fabric is {} and its a {} wear", my_cloth_type, my_wear_location );


//Now below will not work,as t-shirt is partially moved
// println!("{:?}", t_shirt);

//Below will also not work as, this property moved already
// println!("{}",t_shirt.cloth_type);

//But this works as only reference is transferred
println!("wear location : {}", t_shirt.wear_location);
}

fn borrowing(){
    let a = String::from("hello");
    //borrowing a value, (a is giving to b)
    let b = &a;

    //borrowing, taking method
    let ref c = a;

    println!("a: {}, b: {}, c: {}", a,*b,*c);

   
    let mut greetings = String::from("Good Morning");

    let evening_greetings = &mut greetings;
    
    *evening_greetings = String::from("Good Evening");

    //Once the mutability is assigned to variable
    //only that variable should continusly be used for the scope in conti
     greetings = "Good night".to_owned();

    println!("{}",greetings );


}

fn main() {
    // partial_borrow();
    borrowing();
}
