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

// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // Using `Self` to fill in the blank.
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }

//     // Fill in the blank, DON'T use any variants of `Self`.
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {
//     println!("Success!");
// }

//4.

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. Implement an associated function `new`,
//     // 2. It will return a TrafficLight contains color "red"
//     // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
//     pub fn new() ->TrafficLight{
//         TrafficLight{
//             color : "red".to_string()
//         }
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light: TrafficLight = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");

//     println!("Success!");
// }

//5.


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // Using multiple `impl` blocks to rewrite the code below.
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }



// fn main() {
//     println!("Success!");
// }

//6.


#[derive(Debug)]

enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self)-> &str{
        match self {
            TrafficLightColor::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "greeen",
        }
    }
    
}

fn main() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}