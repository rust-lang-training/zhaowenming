use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, u32> {
    // 创建一个哈希表来存储字母的计数
    let mut counts = HashMap::new();
    
    // 将文本转换为小写并迭代每个字符
    for c in text.to_lowercase().chars() {
        // 仅统计 a-z 之间的字符
        if c.is_ascii_alphabetic() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    
    counts
}

fn main() {
    let sample_text = "zhao wenming";
    let counts = count_letters(sample_text);
    
    // 打印每个字母的计数
    for letter in 'a'..='z' {
        println!("{}: {}", letter, counts.get(&letter).unwrap_or(&0));
    }
}

