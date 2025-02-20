
// // FILL in the blanks and FIX the errors
// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>{
//     let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
//     let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();

//     Ok(n1.unwrap() * n2.unwrap())
// }

// fn main() {
//     let result:Result<i32, ParseIntError> = multiply("10", "2");
//     assert_eq!(result, Ok(20));
    
//     let result = multiply("4", "2");
//     assert_eq!(result, Ok(8));

//     println!("Success!");
// }

//2


// use std::num::ParseIntError;

// // IMPLEMENT multiply with ?
// // DON'T use unwrap here
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 : i32 = n1_str.parse::<i32>()?;  //Ok(3) -> 3    ? unwraped it
//     let n2 : i32 = n2_str.parse::<i32>()?;  //Ok(4) -> 4

//     Ok(n1 * n2) // 12 -> Ok(12)
// }

// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!");
// }


//3


// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f: Result<File, io::Error> = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s: String = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// // FILL in the blanks with one code line
// // DON'T change any code lines
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!");
// }

//4

// use std::num::ParseIntError;

// // FILL in the blank in two ways: map, and_then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//   // n_str.parse::<i32>().map(|n| n+2)
//    n_str.parse::<i32>().and_then(|n| Ok(n+2))
// }

// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);

//     println!("Success!");
// }

//5

// use std::num::ParseIntError;

// // With the return type rewritten, we use pattern matching without `unwrap()`.
// // But it's so Verbose...
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1)  => {
//             match n2_str.parse::<i32>() {
//                 Ok(n2)  => {
//                     Ok(n1 * n2)
//                 },
//                 Err(e) => Err(e),
//             }
//         },
//         Err(e) => Err(e),
//     }
// }

// // Rewriting `multiply` to make it succinct
// // You should use BOTH of  `and_then` and `map` here.
// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     n1_str
//         .parse::<i32>()
//         .and_then(|n1| n2_str.parse::<i32>()
//         .map(|n2| n1 * n2))
// }

// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     // This still presents a reasonable answer.
//     let twenty = multiply1("10", "2");
//     print(twenty);

//     // The following now provides a much more helpful error message.
//     let tt = multiply("t", "2");
//     print(tt);

//     println!("Success!");
// }

//6.

// use std::num::ParseIntError;

// // FILL in the blank
// type Res<i32> = Result<i32, ParseIntError>;

// // Use the above alias to refer to our specific `Result` type.
// fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
//     })
// }

// // Here, the alias again allows us to save some space.
// fn print(result: Res<i32>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     print(multiply("10", "2"));
//     print(multiply("t", "2"));

//     println!("Success!");
// }

//7.


use std::num::ParseIntError;

type Res = Result<(), ParseIntError>;
fn main() -> Res {
    let number_str:&str = "10";
    let number: i32 = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}