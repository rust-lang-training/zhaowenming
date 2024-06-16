use rand::Rng;
fn main() {
    println!("请随机生成10个整数");
    let mut arr = [0u32; 10];
    for i in 0..10 {
        arr[i] = rand::thread_rng().gen_range(1..100);
    }

    println!("BEFORE: {:?}", arr);
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    println!("AFTER: {:?}", arr);
}


