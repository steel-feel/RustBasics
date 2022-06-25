mod features;

pub fn intro() {
    println!("Welcome to bird module");
    println!("It can {}",features::feature_1());
}
