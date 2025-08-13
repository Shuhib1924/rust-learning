use std::mem::size_of_val;
use std::any::type_name_of_val;
use std::ops::{Range, RangeInclusive};

fn main() {
    // println!("\nValue ranges of signed integers:");
    // println!("i8   from {} to {} (size: {} bytes)", i8::MIN, i8::MAX, size_of::<i8>());
    // println!("i16  from {} to {} (size: {} bytes)", i16::MIN, i16::MAX, size_of::<i16>());
    // println!("i32  from {} to {} (size: {} bytes)", i32::MIN, i32::MAX, size_of::<i32>());
    // println!("i64  from {} to {} (size: {} bytes)", i64::MIN, i64::MAX, size_of::<i64>());
    // println!("i128 from {} to {} (size: {} bytes)", i128::MIN, i128::MAX, size_of::<i128>());
    // println!("isize from {} to {} (size: {} bytes)", isize::MIN, isize::MAX, size_of::<isize>());
    // println!("\nValue ranges of unsigned integers:");
    // println!("u8   from {} to {} (size: {} bytes)", u8::MIN, u8::MAX, size_of::<u8>());
    // println!("u16  from {} to {} (size: {} bytes)", u16::MIN, u16::MAX, size_of::<u16>());
    // println!("u32  from {} to {} (size: {} bytes)", u32::MIN, u32::MAX, size_of::<u32>());
    // println!("u64  from {} to {} (size: {} bytes)", u64::MIN, u64::MAX, size_of::<u64>());
    // println!("u128 from {} to {} (size: {} bytes)", u128::MIN, u128::MAX, size_of::<u128>());
    // println!("usize from {} to {} (size: {} bytes)", usize::MIN, usize::MAX, size_of::<usize>());
    // println!("\nValue ranges of floating point numbers:");
    // println!("f32  from {} to {} (size: {} bytes)", f32::MIN, f32::MAX, size_of::<f32>());
    // println!("f64  from {} to {} (size: {} bytes)", f64::MIN, f64::MAX, size_of::<f64>());

    // // print
    // let x: f64 = 2.5;
    // println!("The value of x is: {}", x.floor());
    // println!("The value of x is: {}", x.ceil());
    // println!("The value of x is: {}", x.round());
    // println!("The value of x is: {}", x.fract());
    // println!("x with 4 digits of x is: {:.4}", x);

    // casting
    // let number: i32 = 40;
    // println!("The value of number is: {}", number);
    // println!("The type of number is: {}", type_name_of_val(&number));
    // println!("The size of number is: {} bytes", size_of_val(&number));
    // let minimized_number = number as i8;
    // println!("The value of minimized_number is: {}", minimized_number);
    // println!("The type of minimized_number is: {}", type_name_of_val(&minimized_number));
    // println!("The size of minimized_number is: {} bytes", size_of_val(&minimized_number));

    // bool
    // let is_active: bool = true;
    // println!("The value of is_active is: {}", is_active);
    // println!("The type of is_active is: {}", type_name_of_val(&is_active));
    // println!("The size of is_active is: {} bytes", size_of_val(&is_active));
    // let num1 = 1;
    // let is_num1 = num1 > 5;
    // println!("The value of is_num1 is: {}", is_num1);
    // println!("The type of is_num1 is: {}", type_name_of_val(&is_num1));
    // println!("The size of is_num1 is: {} bytes", size_of_val(&is_num1));
    // let num2 = 4;
    // let is_num2 = !num2;
    // println!("The value of is_num2 is: {}", is_num2);
    // println!("The type of is_num2 is: {}", type_name_of_val(&is_num2));
    // println!("The size of is_num2 is: {} bytes", size_of_val(&is_num2));


    // # char
    // let char1 = 'a';
    // let char2 = '🚀';
    // println!("The value of char1 is: {}", char1.is_alphabetic());
    // println!("The value of char2 is: {}", char2.is_alphabetic());
    // println!("The value of char1 is: {}", char1.is_numeric());
    // println!("The value of char2 is: {}", char2.is_numeric());
    // println!("The value of char1 is: {}", char1.is_uppercase());
    // println!("The value of char2 is: {}", char2.is_uppercase());
    // println!("The value of char1 is: {}", char1.is_lowercase());
    // println!("The value of char2 is: {}", char2.is_lowercase());

    // array
    // let array1: [i8; 5] = [1, 2, 3, 4, 5];
    // println!("The value of array1 is: {:#?}", array1);
    // println!("The type of array1 is: {}", type_name_of_val(&array1));
    // println!("The size of array1 is: {} bytes", size_of_val(&array1));
    // let array2: [i16; 5] = [1, 2, 3, 4, 5];
    // println!("The value of array2 is: {:#?}", array2);
    // println!("The type of array2 is: {}", type_name_of_val(&array2));
    // println!("The size of array2 is: {} bytes", size_of_val(&array2));
    // let array3: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("The value of array3 is: {:#?}", array3);
    // println!("The type of array3 is: {}", type_name_of_val(&array3));
    // println!("The size of array3 is: {} bytes", size_of_val(&array3));
    // let array4: [i64; 5] = [1, 2, 3, 4, 5];
    // println!("The value of array4 is: {:#?}", array4);
    // println!("The type of array4 is: {}", type_name_of_val(&array4));
    // println!("The size of array4 is: {} bytes", size_of_val(&array4));
    // let array5: [i128; 5] = [1, 2, 3, 4, 5];
    // println!("The value of array5 is: {:#?}", array5);
    // println!("The type of array5 is: {}", type_name_of_val(&array5));
    // println!("The size of array5 is: {} bytes", size_of_val(&array5));
    // let array6: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    // println!("The value of array6 is: {:#?}", array6);
    // println!("The type of array6 is: {}", type_name_of_val(&array6));
    // println!("The size of array6 is: {} bytes", size_of_val(&array6));
    // let array7: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    // println!("The value of array7 is: {:#?}", array7);
    // println!("The type of array7 is: {}", type_name_of_val(&array7));
    // println!("The size of array7 is: {} bytes", size_of_val(&array7));
    // let array8: [bool; 5] = [true, false, true, false, true];
    // println!("The value of array8 is: {:#?}", array8);
    // println!("The type of array8 is: {}", type_name_of_val(&array8));
    // println!("The size of array8 is: {} bytes", size_of_val(&array8));
    // let array9: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    // println!("The value of array9 is: {:#?}", array9);
    // println!("The type of array9 is: {}", type_name_of_val(&array9));
    // println!("The size of array9 is: {} bytes", size_of_val(&array9));
    // let array10: [&str; 5] = ["a", "b", "c", "d", "e"];
    // println!("The value of array10 is: {:#?}", array10);
    // println!("The type of array10 is: {}", type_name_of_val(&array10));
    // println!("The size of array10 is: {} bytes", size_of_val(&array10));
    // let array11: [String; 5] = [String::from("a"), String::from("b"), String::from("c"), String::from("d"), String::from("e")];
    // println!("The value of array11 is: {:#?}", array11);
    // println!("The type of array11 is: {}", type_name_of_val(&array11));
    // println!("The size of array11 is: {} bytes", size_of_val(&array11));
    // let array12: [String; 5] = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
    // println!("The value of array12 is: {:#?}", array12);
    // println!("The type of array12 is: {}", type_name_of_val(&array12));
    // println!("The size of array12 is: {} bytes", size_of_val(&array12));

    // array slice
    // let array13: [i8; 5] = [1, 2, 3, 4, 5];
    // let array14: &[i8] = &array13[1..3];
    // println!("The value of array14 is: {:#?}", array14);
    // println!("The type of array14 is: {}", type_name_of_val(&array14));
    // println!("The size of array14 is: {} bytes", size_of_val(&array14));

    // array: add & remove
    // let mut array15: [i8; 5] = [1, 2, 3, 4, 5];
    // array15[0] = 10;
    // println!("The value of array15 is: {:#?}", array15);
    // println!("The type of array15 is: {}", type_name_of_val(&array15));
    // println!("The size of array15 is: {} bytes", size_of_val(&array15));

    // dbg!(2+5);

    // tuple
    // let tuple1: (i8, i16, i32, i64, i128, isize) = (1, 2, 3, 4, 5, 6);
    // println!("The value of tuple1 is: {tuple1:#?}");
    // println!("The type of tuple1 is: {}", type_name_of_val(&tuple1));
    // println!("The size of tuple1 is: {} bytes", size_of_val(&tuple1));

    // tuple: access
    // let tuple2: (i8, i16, i32, i64, i128, isize) = (1, 2, 3, 4, 5, 6);
    // let (a, b, c, d, e, f) = tuple2;
    // println!("The value of tuple2 is: {tuple2:#?}");
    // println!("The type of tuple2 is: {}", type_name_of_val(&tuple2));
    // println!("The size of tuple2 is: {} bytes", size_of_val(&tuple2));
    // println!("The value of a is: {}", a);
    // println!("The value of b is: {}", b);
    // println!("The value of c is: {}", c);
    // println!("The value of d is: {}", d);
    // println!("The value of e is: {}", e);
    // println!("The value of f is: {}", f);

    // range & range iteration
    // let range1: Range<i8> = 1..5;
    // println!("The value of range1 is: {range1:#?}");
    // println!("The type of range1 is: {}", type_name_of_val(&range1));
    // println!("The size of range1 is: {} bytes", size_of_val(&range1));
    // for i in range1 {
    //     println!("The value of i is: {}", i);
    // }
    // let range2: std::ops::RangeInclusive<i8> = 1..=5;
    // println!("The value of range2 is: {range2:#?}");
    // println!("The type of range2 is: {}", type_name_of_val(&range2));
    // println!("The size of range2 is: {} bytes", size_of_val(&range2));
    // for i in range2 {
    //     println!("The value of i is: {}", i);
    // }
    // let range3: RangeInclusive<char> = 'a'..='e';
    // println!("The value of range3 is: {range3:#?}");
    // println!("The type of range3 is: {}", type_name_of_val(&range3));
    // println!("The size of range3 is: {} bytes", size_of_val(&range3));
    // for i in range3 {
    //     println!("The value of i is: {}", i);
    // }

    // generics
    // let generic1: i8 = 5;
    // println!("The value of generic1 is: {}", generic1);
    // println!("The type of generic1 is: {}", type_name_of_val(&generic1));
    // println!("The size of generic1 is: {} bytes", size_of_val(&generic1));

    // type inference
    // let x = 2;
    // println!("The value of x is: {}", x);
    // println!("The type of x is: {}", type_name_of_val(&x));
    // println!("The size of x is: {} bytes", size_of_val(&x));
    // let y:i8 = x;
    // println!("The value of y is: {}", y);
    // println!("The type of y is: {}", type_name_of_val(&y));
    // println!("The size of y is: {} bytes", size_of_val(&y));
   }