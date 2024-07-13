struct ArrayUtils {}

fn main() {
    impl ArrayUtils {
        fn max_value(slice: &[i32]) -> i32 {
            *slice.iter().max().expect("Slice is empty")
        }
        fn min_value(slice: &[i32]) -> i32 {
            *slice.iter().min().expect("Slice is empty")
        }

        fn max_value_ref(slice: &[i32]) -> &i32 {
            slice.iter().max().expect("Slice is empty")
        }

        fn min_value_ref(slice: &[i32]) -> &i32 {
            slice.iter().min().expect("Slice is empty")
        }

        fn average_value(slice: &[i32]) -> f64 {
            let sum: i32 = slice.iter().sum();
            let count = slice.len();
            sum as f64 / count as f64
        }
        fn double_elements(slice: &mut [i32]) {
            for element in slice.iter_mut() {
                *element *= 2;
            }
        }
    }

    let mut array = [1, 2, 3, 4, 5];

    // 最大值
    let max_val = ArrayUtils::max_value(&array);
    println!("Max value: {}", max_val);

    // 最小值
    let min_val = ArrayUtils::min_value(&array);
    println!("Min value: {}", min_val);

    // 最大值的引用
    let max_val_ref = ArrayUtils::max_value_ref(&array);
    println!("Max value reference: {}", max_val_ref);

    // 最小值的引用
    let min_val_ref = ArrayUtils::min_value_ref(&array);
    println!("Min value reference: {}", min_val_ref);

    // 平均值
    let avg_val = ArrayUtils::average_value(&array);
    println!("Average value: {}", avg_val);

    // 放大每个元素到原来的2倍
    ArrayUtils::double_elements(&mut array);
    println!("Array after doubling elements: {:?}", array);
}