// use stdin::io;
fn main() {
    // println!("请输入一个正整数");

    // let mut num = 2u32;
    // io.stdin().read_line(mut num).expect("输入数字无效");
    // let mut num:u32 = num.trim().parse().expect("输入数字无效");

    // while num*num
    let bool = is_prime(23);

    println!("{}", bool)
}

fn is_prime(n: u64) -> bool {
    let mut num = 2u64;
    while num * num <= n {
        if n % num == 0 {
            return false;
        }
        num += 1
    }
    true
}
