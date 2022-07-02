mod bird;
//Imported local create from '11_crates' folder
//use could limit to crate, module or down to function 
use crates::lib_nat::intro;

fn main() {

bird::intro();

//Called via long path form
//crates::lib_nat::intro();

//Called via short form
intro();
}
