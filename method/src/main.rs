// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // Complete the area method which return the area of a Rectangle.
//     fn area(self)->u32{
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1: Rectangle = Rectangle { width: 30, height: 50 };

//     assert_eq!(rect1.area(), 1500);

//     println!("Success!");
// }

//2.

// Only fill in the blanks, DON'T remove any line!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // Don't take the ownership of `light` here.
//     light.show_state();
//     // ... Otherwise, there will be an error below
//     println!("{:?}", light);
// }

//3.

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}