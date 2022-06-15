fn main() {
   let name = "John Hopkins";
   let mut age = 21;

    {
        let school_name = "St. Josph Convent";
    }
    
    age = 22;
    //Can't access school_name here :(

    println!("Student name is {}. age is {}",name,age );    


}
