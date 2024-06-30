// use std::rc::Rc;
use std::cell::RefCell;
// fn main() {
//     // println!("Hello, world!");
//     let s = String::from("hello");
//     takes_ownership(s);
//     // s = 123
//     let x = 5;

//     // x = 345;
//     makes_copy(x)
// }

// fn takes_ownership(some_string:String) {
//     println!("{}",some_string)
// }

// fn makes_copy(some_integer:i32){
//     println!("{}",some_integer)
// }

// fn main() {
//     let names = [
//         String::from("John"),
//         String::from("Tom"),
//         String::from("Penny"),
//         String::from("Sheldon"),
//     ];

//     for i in 0..4 {
//         let s = &names[i];
//         println!("{}",s);
//     }

//     println!("names[0] = {}", &names[0]);
// }


fn main() {
    // let s = String::from("Hello world");
    // let bytes = s.into_bytes();
    // println!("{}",s);
    // let x = 10;
    // let y = &x;
    // assert!(*y ==10);


    // let mut x = 10;
    // let y = &mut x;
    // *y = 20;



    // let s1 = String::from("hello");
    // let len = calculate_length(&&s1);
    // println!("The length of '{}' is {}.",s1,len)



    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{},{}, and {}," r1,r2,r3);

    // let a: &str = "hellp"
    // longest(a);


    // let mut s = String::from("Hello World");
    // let rs = &s;
    // s.push_str(" I'm rust");

    // println!("The string is: {}", rs)




    // let s1 = String::from("Hellp world");
    // let rs1 = &s1;
    // let s2 = s1;
    // println!("string is: {}", rs1)




    // let bytes = gen_string().as_bytes();
    // println!("bytes are: {:?}",bytes)
    // RTFM




    // let s: Rc<String> = Rc::new(String::from("shirataki"));
    // let t: Rc<String> = s.clone();
    
    // println!("ref count: {}", Rc::strong_count(&s));
    // {
    //   let t: Rc<String> = s.clone();
    //   println!("ref count: {}", Rc::strong_count(&t));  
    // }
    // let u: Rc<String> = s.clone();
    // println!("ref count: {}", Rc::strong_count(&s));
    // println!("ref count: {}", Rc::strong_count(&u));




    let s = RefCell::new(String::from("Hello world!"));
    append_string(&s);
    println!("s is: {}", s.borrow());

}
fn append_string(s: &RefCell<String>) {
    let rs = s.borrow();
    let mut ms = s.borrow_mut();
    (*ms).push_str(" I'm Rust");

    println!("rs: {}", rs);
}


// fn gen_string() -> String {
//     String::from("Hellp world!")
// }

// fn calculate_length(s:&String) -> usize {
//     s.len()
// }


// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }



