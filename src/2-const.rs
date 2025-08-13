use std::any::type_name;
use std::any::type_name_of_val;
use std::mem::{size_of, size_of_val};

// const TAX_RATE: f64 = 19.0;

// fn main() {
//     let price = 100.0;
//     let tax = price * TAX_RATE / 100.0;
//     println!("The tax is: {}", tax);
// }

type MyNewType = i32;

fn main() {
    println!("The type of MyNewType is: {}", type_name::<MyNewType>());
    println!("The size of MyNewType is: {} bytes", size_of::<MyNewType>());
    let x: MyNewType = 5;
    println!("The value of x is: {}", x);
    println!("The type of x is: {}", type_name_of_val(&x));
    println!("The size of x is: {} bytes", size_of_val(&x));
}