// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);

//     // FIX the error in two ways
//     /* 1. use a similar type which `impl From<char>`, maybe you
//     should check the docs mentioned above to find the answer */
//     // 2. a keyword from the last chapter
//     let i3: u32 = 'a'.into();

//     // FIX the error in two ways
//     //let s: String = String::from('a');

//     let s:String = 'a'.into();
//     println!("Success!");
// }

//2.

// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     // IMPLEMENT `from` method
//     fn from(value: i32) -> Self {
//         Self { value }
//     }
// }

// // FILL in the blanks
// fn main() {
//     let num = Number::from(30);
//     assert_eq!(num.value, 30);

//     let num: Number = 30.into();
//     assert_eq!(num.value, 30);

//     println!("Success!");
// }


//3.

// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     // IMPLEMENT from method
//     fn from(err: io::Error) -> Self {
//         Self::IoError(err)
//     }
// }

// impl From<num::ParseIntError> for CliError {
//     // IMPLEMENT from method
//     fn from(err: num::ParseIntError) -> Self {
//         Self::ParseError(err)
//     }
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!");
// }

//4

// // TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// // use std::convert::TryInto;

// fn main() {
//     let n: i16 = 256;

//     // Into trait has a method `into`,
//     // hence TryInto has a method ?
//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, 0);

//     println!("Success!");
// }

//5.

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!");
}