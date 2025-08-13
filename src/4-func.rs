// fn main() {
//     fn1();
//     fn2();
//     fn3();
// }

// fn fn1() {
//     println!("fn1");
// }
// fn fn2() {
//     println!("fn2");
// }
// fn fn3() {
//     println!("fn3");
// }

// # with args without return
// fn main() {
//     fn1(1,2);
//     fn2(18,"Shuhailat");
//     fn3(vec!["Shuhailat","Shuhaib", "Ahmad", "Ali"]); // fn3(vec!["Shuhailat","Shuhaib"]); vec!["Shuhailat","Shuhaib"]
// }

// fn fn1(x: i32, y: i32) {
//     println!("{}",x+y);
// }
// fn fn2(age: i8, name: &str) {
//     println!("my name is {} and i am {} years old",name,age);
// }
// fn fn3(users: Vec<&str>) {
//     for (index, user) in users.into_iter().enumerate() {
//         println!("{}: {}",index+1,user);
//     }
// }

// ^ function with args and return

// fn main(){
//     let result = square(5);
//     println!("the result is {}",result);
// }

// fn square(num: i16)-> i16{
//     num*num
// }

// * call function as variable
// fn main(){
//     let num1 = 6;
//     let calculation = {
//         let num2 = 7;
//         num1 * num2
//     };
//     println!("{calculation}");
// }

// % little project
// fn apply(num: i32, title: &str){
//     println!("i'm applying to {num} {title} job");
// }

// fn even(num: i32) -> bool{
//     num % 2 == 0
// }

// fn containing(text: &str)-> (bool, bool){
//     (text.contains("a"), text.contains("z"))
// }

// fn main(){
//     apply(20,"Rust Dev");
//     let num = 10;
//     let result = even(num);
//     println!("{}",result);
//     println!("{:?}",containing("ali"));
//     println!("{:?}",containing("zoro"));
//     println!("{:?}",containing("zalando"));
// }