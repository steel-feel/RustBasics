fn main() {

   let decimal_number = 45.643;
   //type conversion
   let int_number = decimal_number as u8;

   println!("Decimal Number {}", decimal_number);
   println!("Casted Number {}", int_number);

   //inference
   let big_number:u64 = 12343534563445656646;

   type Color = u32;

   println!("Big Number {}", big_number);

   let my_color:Color = 0x12ff65; 

   //0x to print in hex format
   println!("My faviorate color is {:0x}",my_color )
}
