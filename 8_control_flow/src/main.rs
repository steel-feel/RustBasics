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

fn main() {
    //If-else example
    if_else('m');
    //loop example
     println!( "last value {}", multiplicatio_table(2,10));
    //while
    while_fibonacci(40);

}

