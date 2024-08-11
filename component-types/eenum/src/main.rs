// //1
// // Fix the errors
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }

// // C-like enum
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }


// fn main() {
//     // An enum variant can be converted to a integer by `as`
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as isize, Number2::One as isize);

//     println!("Success!1");
// } 

//2.

//Fill in the blank
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move{x:1, y:2}; // Instantiating with x = 1, y = 2 
    let msg2 = Message::Write(String::from("hello world")); // Instantiating with "hello, world!"

    println!("Success!2");
} 

//3.
