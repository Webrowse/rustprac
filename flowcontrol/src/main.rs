// // Fill in the blanks
// fn main() {
//     let n = 5;

//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// } 
//-------------------------------------------


// // Fix the errors
// fn main() {
//     let n = 5;

//     let big_n =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");

//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");

//             n / 2 
//         };

//     println!("{} -> {}", n, big_n);
// } 
//---------------------------------------------------

// fn main() {
//     for n in 1..100 { // modify this line to make the code work
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }

//     println!("Success!");
// } 

//---------------------------------------------------


// Fix the errors without adding or removing lines
fn main() {
    let names: [String; 2] = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers:[i32;3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in &numbers {
        // Do something with n...
    }
    
    println!("{:?}", numbers);
} 