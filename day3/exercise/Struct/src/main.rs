// struct Point(f32,f32);
// fn main() {
//     // println!("Hello, world!");
//     // let p = Point(1.0)
// }

// const user = User {
//     active:true,
//     username:"zhang",
//     email:"",
//     sex:"男"
// }

// let User {
//     active,
//     username,
//     email,
//     sex
// } = user


// println!("username is:{}", username)


struct Rectangle {
    width:f32,
    height:f32,
}

impl Rectangle {
    fn square(size: f32) -> Self {
        Self {
            width:size,
            height:size
        }
    }

    fn new(width:f32, height:f32) -> Self {
        Self {
            width,
            height
        }
    }

    fn permiter(&self) -> f32 {
        (self.width + self.height) * 2.0f32
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn scale(&mut self,width_scale:f32,height_scale:f32) {
        self.width = self.width * self.width_scale;
        self.height = self.height*self.height_scale
    }
}
