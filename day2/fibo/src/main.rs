use std::io;
fn main() {
    println!("请输入多少项");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("数据读取失败");
    let n: u32 = n.trim().parse().expect("请输入有效的整数");
    let total =  fibo_loop(n);//fibo1(n);
    println!("{}", total)
}

fn fibo1(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    fibo1(n - 1) + fibo1(n - 2)
}

fn fibo_loop(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }

    let mut a = 1u32;
    let mut b = 1u32;

    for _ in 3..n {
        let m = a + b;
        a = b;
        b = m;
    }
    a + b
}
