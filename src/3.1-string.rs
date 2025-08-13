use std::mem::size_of_val;
use std::any::type_name_of_val;
fn main(){
    //@ declared
    // let a = String::new();
    //@ initialized
    // let b = String::from("hello");
    // println!("{}", a);
    // println!("{}", a.capacity());
    // println!("{}", a.len());
    // println!("{}", type_name_of_val(&a));
    // println!("{}", size_of_val(&a));
    // println!("{}", b);
    // println!("{}", b.capacity());
    // println!("{}", b.len());
    // println!("{}", type_name_of_val(&b));
    // println!("{}", size_of_val(&b));

    //@ push
    // let mut c = String::from("hello");
    // c.push('!');
    // println!("{}", c);
    // println!("{}", c.capacity());
    // println!("{}", c.len());
    // println!("{}", type_name_of_val(&c));
    // println!("{}", size_of_val(&c));

    //@ push_str
    // let mut d = String::from("hello");
    // d.push_str(" world");
    // println!("{}", d);
    // println!("{}", d.capacity());
    // println!("{}", d.len());
    // println!("{}", type_name_of_val(&d));
    // println!("{}", size_of_val(&d));

    //@ +
    // let mut e = String::from("hello");
    // e += " world";
    // println!("{}", e);
    // println!("{}", e.capacity());
    // println!("{}", e.len());
    // println!("{}", type_name_of_val(&e));
    // println!("{}", size_of_val(&e));

    //@ format
    // let mut f = String::from("hello");
    // f = format!("{} world", f);
    // println!("{}", f);
    // println!("{}", f.capacity());
    // println!("{}", f.len());
    // println!("{}", type_name_of_val(&f));
    // println!("{}", size_of_val(&f));

    //@ ownership
    // let mut g = String::from("hello");
    // let h = g;
    // println!("{}", h);
    // println!("{}", h.capacity());
    // println!("{}", h.len());
    // println!("{}", type_name_of_val(&h));
    // println!("{}", size_of_val(&h));
    // println!("{}", g);
    // println!("{}", g.capacity());
    // println!("{}", g.len());
    // println!("{}", type_name_of_val(&g));
    // println!("{}", size_of_val(&g));

    //@ clone
    // let i = String::from("hello");
    // let j = i.clone();
    // println!("{}", j);
    // println!("{}", j.capacity());
    // println!("{}", j.len());
    // println!("{}", type_name_of_val(&j));
    // println!("{}", size_of_val(&j));
    // println!("{}", i);
    // println!("{}", i.capacity());
    // println!("{}", i.len());
    // println!("{}", type_name_of_val(&i));
    // println!("{}", size_of_val(&i));

    // drop()
    // let s = String::from("hello");
    // drop(s); // s is dropped here
    // println!("{}", s); // s is moved here


    // adress in stack
    // let s1 = String::from("hello");
    // println!("value: {}, addr in stack: {:p}, addr of bytes (heap): {:p}, type: {}, size: {}", s1, &s1, s1.as_ptr(), type_name_of_val(&s1), size_of_val(&s1));

    // // address in heap
    // let s2 = 5;
    // println!("value: {}, addr in stack: None, addr of bytes (heap): None, type: {}, size: {}", s2, type_name_of_val(&s2), size_of_val(&s2));

    //@ reference
    // let s = String::from("hello"); // String with 24 bytes
    // let s_clone = s.clone();
    // let r1: &String = &s; // reference to String with 8 bytes
    // let r1_clone = r1.clone();
    // let r2: &str = &s.as_str(); // &str with 16 bytes
    // let r2_clone = r2.clone();
    // println!("value: {}, type: {}, size: {}, addr: {:p}, addr of bytes (heap): {:p}, len: {}, cap: {}", s, type_name_of_val(&s), size_of_val(&s), &s, s.as_ptr(), s.len(), s.capacity());
    // println!("value: {}, type: {}, size: {}, addr: {:p}, addr of bytes (heap): {:p}, len: {}, cap: {}", s_clone, type_name_of_val(&s_clone), size_of_val(&s_clone), &s_clone, s_clone.as_ptr(), s_clone.len(), s_clone.capacity());
    // println!("value: {}, type: {}, size: {}, addr: {:p}, addr of bytes (heap): {:p}, len: {}, cap: {}", r1, type_name_of_val(&r1), size_of_val(&r1), r1, r1.as_ptr(), r1.len(), r1.capacity());
    // println!("value: {}, type: {}, size: {}, addr: None, addr of bytes (heap): {:p}, len: {}, cap: {}", r1_clone, type_name_of_val(&r1_clone), size_of_val(&r1_clone), r1_clone.as_ptr(), r1_clone.len(), r1_clone.capacity());
    // println!("value: {}, type: {}, size: {}, addr: {:p}, addr of bytes (heap): {:p}, len: {}, cap: None", r2, type_name_of_val(&r2), size_of_val(&r2), r2, r2.as_ptr(), r2.len());
    // println!("value: {}, type: {}, size: {}, addr: {:p}, addr of bytes (heap): {:p}, len: {}, cap: None", r2_clone, type_name_of_val(&r2_clone), size_of_val(&r2_clone), r2_clone, r2_clone.as_ptr(), r2_clone.len());

    // // reference 2
    // let stack_value: i32 = 5;
    // println!("value: {}, addr: None, addr of bytes (heap): None, type: {}, size: {}", stack_value, type_name_of_val(&stack_value), size_of_val(&stack_value));
    // let stack_value_ref: &i32 = &stack_value;
    // println!("value: {}, addr in stack: {:p}, addr of bytes (heap): None, type: {}, size: {}", stack_value_ref, stack_value_ref, type_name_of_val(&stack_value_ref), size_of_val(&stack_value_ref));

    // let heap_value: String = String::from("hello");
    // let heap_value_ref: &String = &heap_value;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): {:p}, type: {}, size: {}", heap_value, heap_value_ref, heap_value.as_ptr(), type_name_of_val(&heap_value), size_of_val(&heap_value)); 
    // println!("value: {}, addr: {:p}, addr of bytes (heap): {:p}, type: {}, size: {}", heap_value_ref, heap_value_ref, heap_value_ref.as_ptr(), type_name_of_val(&heap_value_ref), size_of_val(&heap_value_ref));

    // de-reference
    // let value = 32;
    // println!("value: {}, addr: None, addr of bytes (heap): None, type: {}, size: {}", value, type_name_of_val(&value), size_of_val(&value));
    // let ref_value = &value;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", ref_value, ref_value, type_name_of_val(&ref_value), size_of_val(&ref_value));
    // println!("De-referenced - value: {}, type: {}, size: {}", *ref_value, type_name_of_val(ref_value), size_of_val(ref_value));

    // // reference to reference
    // let ref_ref_value = &ref_value;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", ref_ref_value, ref_ref_value, type_name_of_val(&ref_ref_value), size_of_val(&ref_ref_value));
    // println!("De-referenced 2 times - value: {}, type: {}, size: {}", **ref_ref_value, type_name_of_val(ref_ref_value), size_of_val(ref_ref_value));

    // // reference to reference to reference
    // let ref_ref_ref_value = &ref_ref_value;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", ref_ref_ref_value, ref_ref_ref_value, type_name_of_val(&ref_ref_ref_value), size_of_val(&ref_ref_ref_value));
    // println!("De-referenced 3 times - value: {}, type: {}, size: {}", ***ref_ref_ref_value, type_name_of_val(ref_ref_ref_value), size_of_val(ref_ref_ref_value));

    // // reference to reference to reference to reference
    // let ref_ref_ref_ref_value = &ref_ref_ref_value;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", ref_ref_ref_ref_value, ref_ref_ref_ref_value, type_name_of_val(&ref_ref_ref_ref_value), size_of_val(&ref_ref_ref_ref_value));
    // println!("De-referenced 4 times - value: {}, type: {}, size: {}", ****ref_ref_ref_ref_value, type_name_of_val(ref_ref_ref_ref_value), size_of_val(ref_ref_ref_ref_value));

    // String, &String, str, &str
    // let s: String = String::from("hello");
    // let s_ref: &String = &s;
    // let s_ref_ref: &&String = &s_ref;
    // let s_str: &str = "hello";
    // let s_str_ref: &&str = &s_str;
    // let s_static: &'static str = "hello";
    // let s_static_ref: &&str = &s_static;
    // println!("value: {}, addr: None, addr of bytes (heap): {:p}, type: {}, size: {}", s, s.as_ptr(), type_name_of_val(&s), size_of_val(&s));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_ref, s_ref, type_name_of_val(&s_ref), size_of_val(&s_ref));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_ref_ref, s_ref_ref, type_name_of_val(&s_ref_ref), size_of_val(&s_ref_ref));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_str, s_str, type_name_of_val(&s_str), size_of_val(&s_str));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_str_ref, s_str_ref, type_name_of_val(&s_str_ref), size_of_val(&s_str_ref));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_static, s_static, type_name_of_val(&s_static), size_of_val(&s_static));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): None, type: {}, size: {}", s_static_ref, s_static_ref, type_name_of_val(&s_static_ref), size_of_val(&s_static_ref));

    // copy trait reference
    // let s: &str = "hello";
    // let s_ref: &&str = &s;
    // println!("value: {}, addr: {:p}, addr of bytes (heap): {:p}, type: {}, size: {}", s, s.as_ptr(), s_ref, type_name_of_val(&s), size_of_val(&s));
    // println!("value: {}, addr: {:p}, addr of bytes (heap): {:p}, type: {}, size: {}", s_ref, s_ref, s_ref.as_ptr(), type_name_of_val(&s_ref), size_of_val(&s_ref));

    // ownership with function
    // fn take_ownership(s: String) {
    //     println!("{}", s);
    // }
    // let s = String::from("hello");
    // take_ownership(s);
    // println!("{}", s); // error: value borrowed here after move

    // mutable ownership
    // fn take_ownership(mut s: String) {
    //     s.push_str(" world");
    //     println!("{}", s);
    // }
    // let s = String::from("hello");
    // take_ownership(s);
    // println!("{}", s); // error: value borrowed here after move

    // return ownership
    // fn return_ownership() -> String {
    //     let s = String::from("hello");
    //     s
    // }
    // let s2 = return_ownership();
    // println!("{}", s2);

    // fn add_word(mut s: String)->String {
    //     s.push_str("Hello world");
    //     s
    // }
    // let s2 = add_word(String::new());
    // println!("{}", s2);
}