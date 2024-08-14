
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


fn main() {
    let boolean: bool = true;

    // Fill the blank with a match expression:
    //
    
    let binary:u8 = match boolean {
        true =>  1,
        false =>   0
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

