use std::io;

// 辗转相除法计算最大公约数
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("请输入第一个 231-1 范围内的正整数:");
    io::stdin().read_line(&mut input1).expect("读取输入失败");
    
    println!("请输入第二个 231-1 范围内的正整数:");
    io::stdin().read_line(&mut input2).expect("读取输入失败");
    
    // 解析输入并检查是否在范围内
    let num1: u64 = match input1.trim().parse() {
        Ok(n) if n > 0 && n < 2u64.pow(31) - 1 => n,
        _ => {
            println!("输入无效，请输入一个 1 到 231-2 之间的正整数。");
            return;
        }
    };

    let num2: u64 = match input2.trim().parse() {
        Ok(n) if n > 0 && n < 2u64.pow(31) - 1 => n,
        _ => {
            println!("输入无效，请输入一个 1 到 231-2 之间的正整数。");
            return;
        }
    };

    // 计算最大公约数
    let result = gcd(num1, num2);
    println!("{} 和 {} 的最大公约数是: {}", num1, num2, result);
}

