// enum Message {
//     Quit,
//     Move {x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32)
// }
fn main() {
    // println!("Hello, world!");
    // let q = enum.Message::Quit;
    // let m = enum.Message::Move {x:100,y:100};
    // let w = enum.Message::Write("hello");
    // let c = enum.Move::ChangeColor(128,128,128)
    // println!("{}",describe_point(10,20));

    println!("{}",fibo(8)) 
}

// if n == 0 {
//     return 0;
// }
// if n == 1 || n == 2 {
//     return 1;
// }
// fibo1(n - 1) + fibo1(n - 2)
fn fibo(n:u32) -> u32 {
  let num =  match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibo(n-1)+fibo(n-2)
    };
  num
}
// fn describe_point(x: i32, y: i32) -> &'static str {
//     use std::cmp::Ordering::*;
//     match (x.cmp(&0),y.cmp(&0)) {
//         (Equal,Equal) => "at the origin",
//         (_,Equal) => "at the x axis",
//         (Equal,_) => "at the y axis",
//         (Greater,Greater) => "in the first quadrant",
//         (Less,Greater) => "in the secone quadrant",
//         _ => "somewhere else"
//     }
// }


