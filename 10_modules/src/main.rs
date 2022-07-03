mod bird;
//Imported local create from '11_crates' folder
//use could limit to crate, module or down to function 
// use crates_cargo::lib_nat::intro;
use crates_cargo::lib_nat::*;

fn main() {

bird::intro();

//Called via long path form
//crates_cargo::lib_nat::intro();

//Called via short form
intro();
greet_me();

}
