#![allow(dead_code)]

fn if_else(gender:char) {
     //discount for woman 20 and man 10 %, else 5 for everyone
    let discount = if gender == 'M' || gender == 'm' {
        10
    } else if gender == 'F' || gender == 'f'
    {
        20
    } else
    {
        5
    };


    println!("Your discount percentage {}% ", discount);

}

fn multiplicatio_table(num:u32, count: u16) -> u32 {
    let mut i = 0u16;

    'main: loop{
        i += 1;

        if  i > count
            {   
                //this value will be returned as loop final value
                break 'main num * ( (i-1) as u32)  ;
            }

        println!("{} x {} = {}", num, i, num * (i as u32) );
    //notice 'no' semicolon mean, it will be returned;
    }

}

fn while_fibonacci(nums:u16) {

    let mut i:u16 = 3;
    let mut last:u32 = 1;
    let mut second_last:u32 = 1;

    print!("{},{}", last, second_last  );
    let mut tmp:u32;
    while i <= nums {
        tmp = last+  second_last;
        print!(",{}", tmp);

        second_last = last;
        last = tmp;

        i += 1;
    }

}


fn check_word(s: &str,check_letter: char, skip: u32 )-> &str {
    
    let as_bytes = s.as_bytes();
    let mut first_space = 0;
    let mut count = 0;
    let mut last_space = s.len();
    
    
    for (i , &letter) in as_bytes.iter().enumerate() {
      
      if count <= skip  {

        if letter == check_letter as u8 {
            count += 1;
            continue;
        }
        if letter == b' '{
             first_space = i;
        } 
    
      }else{
          if letter == b' '{
             last_space = i;
             break;
        } 
      }
      
    }

    if first_space != 0 {
        first_space += 1 ;
    }

    return &s[first_space..last_space];
    
}

fn main() {
    //If-else example
  //  if_else('m');

  //loop example
   //  println!( "last value {}", multiplicatio_table(2,10));

   //while
  //  while_fibonacci(40);

  //check the word containing letter
//     let s = String::from( "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam enim dolor, bibendum in viverra et, pulvinar at sapien. Etiam vitae volutpat tellus, eget molestie eros. Cras imperdiet turpis a odio rutrum pharetra. Aliquam erat volutpat. Integer et est id elit condimentum hendrerit non eget risus. Integer varius vulputate eros, vel." );
//     let letter = 'c' ;
//    println!("Word containing \"{}\" is {}",letter,check_word(&s, letter ,0));

   
}