
// // Fill the blanks
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire: Direction = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction:: North  => { // Matching South or North here
//             println!("South or North");
//         },
//         _ => println!("west"),
//     };
// }

//2.


// fn main() {
//     let boolean: bool = true;

//     // Fill the blank with a match expression:
//     //
    
//     let binary:u8 = match boolean {
//         true =>  1,
//         false =>   0
//     };

//     assert_eq!(binary, 1);

//     println!("Success!");
// }

//3.


// // Fill in the blanks
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }

//     println!("Success!");
// } 

// fn show_message(msg: Message) {
//     match msg {
//         Message:: Move{x: a , y: b} => { // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
//     }
// }

//4. matches

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
//     }

//     println!("Success!");
// } 

//5. 

// #[derive(Debug)]
// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v : Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//     println!("Before Count: {} and e is {:?}",count, e);
//         if matches!(e, MyEnum::Foo){ //e == MyEnum::Foo { // Fix the error by changing only this line
//             count += 1;
//              println!("After Count: {} and e is {:?}",count, e);
//         }
//     }

//     assert_eq!(count, 2);

//     println!("Success!");
// }

//6.


// fn main() {
//     let o = Some(7);

//     // Remove the whole `match` block, using `if let` instead 
//    if let Some(i) = o {
//     println!("This is a really long string and `{:?}`", i);

//     println!("Success!");
//    }
// }

//7.

// // Fill in the blank
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a : Foo = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);

//         println!("Success!");
//     }
// }


//8. 


// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a: Foo = Foo::Qux(10);


//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others "),
//     }
// }


//9.

// Fix the errors in-place
fn main() {
    let age: Option<i32> = Some(30);
    if let Some(age) = age { 
        
       assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here
    
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }



