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

// //2.

// //Fill in the blank
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1 = Message::Move{x:1, y:2}; // Instantiating with x = 1, y = 2 
//     let msg2 = Message::Write(String::from("hello world")); // Instantiating with "hello, world!"

//     println!("Success!2");
// } 

// //3.

// // Fill in the blank and fix the error
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::Move{x: 1, y: 1};

//     if let Message::Move{x: a, y: b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN！");
//     }

//     println!("Success!3");
// } 

// //4.

// // Fill in the blank and fix the errors
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// } 

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

//5.

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.


fn main() {
    let five: Option<i32> = Option::Some(5);
    let six: Option<i32> = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } else
    {
    panic!("NEVER LET THIS RUN！");
        
    }
        
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}