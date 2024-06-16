use std::io;
fn main() {
    println!("请输入5个整数，空格分隔");

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("数据读取失败");

    let mut numbers = [0u32; 5];
    let mut index = 0;

    for n in s.trim().split(" ") {
        let nn: u32 = n.trim().parse().expect("请输入有效的整数");
        numbers[index] = nn;
        index += 1;

        if index == 5 {
            break;
        }
    }
    let total = arr_add(&numbers[..]);
    println!("sum {:?} = {}", numbers, total);
}

fn arr_add(items: &[u32]) -> u32 {
    let mut total = 0u32;
    for n in items {
        total += n;
    }
    total
}
