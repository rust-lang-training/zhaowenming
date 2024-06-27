use std::{io};
fn main() {
    // println!("Hello, world!");
    // 摄⽒度华⽒度转换

    loop {
        println!("摄氏度转华氏度 输入1");
        println!("华氏度转慑氏度 输入2");
        println!("退出 输入0");

        let mut choose_number = String::new();
        io::stdin().read_line(&mut choose_number).expect("读取输入失败");

        let choose_number = choose_number.trim(); // 去除留白

        if choose_number == "0" {
            break // 退出
        }
        if choose_number == "1" {
            println!("\n请输入摄氏度");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("读取输入失败");
            let c:f32 = c.trim().parse().expect("请输入有效的数值");
            let f = c_to_f(c,choose_number);
            println!("{:.2} 摄氏度-> {:.2} 华氏度\n",c,f);
        }

        if choose_number == "2" {
            println!("\n请输入华氏度");
            let mut h = String::new();
            io::stdin().read_line(&mut h).expect("读取输入失败");
            let h:f32 = h.trim().parse().expect("请输入有效的数值");
            let f = c_to_f(h,choose_number);
            println!("{:.2} 华氏度-> {:.2} 摄氏度\n", h, f);
        }
        
    }
}

fn c_to_f(c:f32,type1: &str) -> f32 {
    if type1 == "1" {
      c * 1.8 + 32.0  
    }else {
      (c-32.0) /1.8
    }
    
}
