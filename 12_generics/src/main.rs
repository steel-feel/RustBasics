use std::fmt::Debug; 

#[derive(Debug)]
struct ANumber<T:Debug>{
    num : T
}



fn PrintANum<T:Debug> (anum:T) {
        println!("{:?}",anum);
    }




fn main() {

    let a_num = ANumber{num:6};

    println!("{:?}",a_num.num);

    PrintANum(a_num);
}
