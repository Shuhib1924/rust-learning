fn main(){
    // let num = 10;
    // if num > 5 {
    //     println!("{} is greater than 5", num);
    // } else if num == 5 {
    //     println!("{} is equal to 5", num);
    // } else {
    //     println!("{} is less than 5", num);
    // }

    // ? even or odd
    // fn even_or_odd(num: i32){
    //     let result = if num % 2 == 0 { "even" } else { "odd" };
    //     println!("{} is {}", num, result);
    // }
    // even_or_odd(10);
    // even_or_odd(11);


    // # match
    // let num = 2;
    // match num {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("other"),
    // }
    
    // let check = true;
    // let response = match check {
    //     true => "Yes",
    //     false => "No",
    // };
    // println!("{}", response);
    
    // let num = 0;
    // match num {
    //     value if value % 2 == 0 => println!("{} is even", value),
    //     value if value % 2 != 0 => println!("{} is odd", value),
    //     _ => println!("{} is not a number", num),
    // }

    // loop
    // increment count
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("current count: {}", count);
    //     if count == 10 {
    //         break;
    //     }
    // }
    // println!("count: {}", count);

    // decrement count
    // let mut count = 10;
    // loop {
    //     count -= 1;
    //     println!("current count: {}", count);
    //     if count == 0 {
    //         break;
    //     }
    // }
    // println!("count: {}", count);

    // // loop with continue
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     if count == 5 {
    //         println!("skipping count: {}", count);
    //         continue;
    //     }
    //     println!("current count: {}", count);
    //     if count == 10 {
    //         break;
    //     }
    // }
    // println!("count: {}", count);

    // while
    // let mut count = 0;
    // while count < 5 {
    //     count += 1;
    // }
    // println!("count: {}", count);

    // // for
    // let mut count = 0;
    // for count in 0..5 {
    //     count += 1;
    // }
    // println!("count: {}", count);

    // recursion
    fn counter(num: i32){
        if num == 0 {
            println!("{num}");
        } else{
            // return num + counter(num - 1);
            println!("{}", num);
            counter(num - 1);
        }
    }
    counter(5);

    // ~project
    // fn color_to_number(color: &str) -> i32 {
    //     match color {
    //         "red" => 1,
    //         "green" => 2,
    //         "blue" => 3,
    //         _ => 0,
    //     }
    // }
    
    // fn factorial_iterative(number: i32) -> i32 {
    //     let mut product = 1; // product = 1
    //     let mut count = number; // count = 5
    
    //     while count > 0 {
    //         product *= count;
    //         count -= 1;
    //     }
    
    //     product
    // }
    
    // fn factorial_recursive(number: i32) -> i32 {
    //     if number == 1 {
    //         return 1;
    //     }
    
    //     number * factorial_recursive(number - 1)
    // }
    // println!("{}", color_to_number("red"));
    // println!("{}", color_to_number("green"));
    // println!("{}", color_to_number("blue"));
    // println!("{}", color_to_number("purple"));

    // println!("{}", factorial_iterative(4));
    // println!("{}", factorial_recursive(4));
}