// Remove something to make it work
#[allow(unused_variables)]
fn main() {
// 1.
    let x: i32 = 5;
    let y: u32 = 5;

    let y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");

// 2.
    let v: u16 = 38_u8 as u16;

    println!("Success!");

//3.

    let x:u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");


// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
// 4.
assert_eq!(i8::MAX, 127); 
assert_eq!(u8::MAX, 255); 

println!("Success4!");

// 5.
let v1 = 251_u32 + 8;
   let v2 = i32::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);


//6.
let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
assert!(v == 1597);

println!("Success6!");

//7.
    let x: f64= 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z:f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    // fn type_of<T>(_: &T) -> String {
    //     format!("{}", std::any::type_name::<T>())
    // }

//8.
assert!(0.1 as f32 +0.2_f32 == 0.3);

println!("Success!8");

//9.
    let mut sum:i32 = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    // for c in 'a'..='z' {
    //     println!("{}",c as u8);
    // }
//10.
use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");

//.11

    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);


}