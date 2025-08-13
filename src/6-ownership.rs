#[allow(unused)]
fn main(){
    //@ move
    // let s = String::from("hello");
    // println!("{}", s); // worked
    // let s2 = s;
    // println!("{}", s2);
    // // println!("{}", s); // is moved

    //@ clone
    // let s = String::from("hello");
    // println!("{}", s); // worked
    // let s2 = s.clone();
    // println!("{}", s2);
    // println!("{}", s); // worked

    //@ copy
    // let x = 5;
    // let y = x;
    // println!("{}", x);
    // println!("{}", y);

    //@ ownership
    // fn takes_ownership(s: String) {
    //     println!("{}", s);
    // }
    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s); // is moved
    
    // fn makes_copy(i: i32) {
    //     println!("{}", i);
    // }
    // let x = 5;
    // makes_copy(x);
    // println!("{}", x); // is cloned

    //@ return ownership
    fn return_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
    let s = String::from("hello");
    let s2 = return_ownership(s);
    println!("{}", s2);
}

