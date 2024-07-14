pub enum TestEnum {
    A,
    B 
}

pub struct Point {
    pub x:f32,
    pub y:f32
}

impl Point {
    pub fn move_pos(&mut self) {
        println!("{}","point")
    }
}

