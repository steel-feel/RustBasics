fn main() {
   let name = "John Hopkins";
   let mut age = 21;

    {
        let _school_name = "St. Josph Convent School";
        println!("Student name is {}. age is {}. Attending {}.",name,age, _school_name ); 
    }
    
    age = 22;
    //Can't access school_name here :(
    println!("Student name is {}. age is {}.",name,age );    


}
