#[derive(Debug)]
struct Binary{
    value: String
}

impl From<i32> for Binary {

    fn from (item : i32) -> Self {

        Binary{value : format!("{:b}", item)}

    }

}

fn main() {
    //Using from
    let binary_4 = Binary::from(4) ;

    // Using into
    let binary_7:Binary = (7).into();

    println!("Binary of 4 is {}", binary_4.value);
    println!("Binary of 7 is {}", binary_7.value);


}
