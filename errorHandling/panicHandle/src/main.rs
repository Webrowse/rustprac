
// FILL the blanks
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // IMPLEMENT the below code
//         panic!()
//      }

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink("lemonade");

//     println!("Exercise Failed if printing out this line!");
// }

//2.

// // MAKE the code work by fixing all panics
// fn main() {
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);

//     let v = vec![1, 2, 3];
//     let ele = v[1];
//     // unwrap may panic when get return a None
//     let ele = v.get(1).unwrap();

//     // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
//     let v = production_rate_per_hour(2);

//     // because of the same reason as above, we have to wrap it in a function to make the panic occur
//     divide(15, 1);

//     println!("Success!")
// }

// fn divide(x:u8, y:u8) {
//     println!("{}", x / y)
// }

// fn production_rate_per_hour(speed: u16) -> f64 {
//     let cph: u16 = 221;
//     match speed {
//         1..=4 => (speed * cph) as f64,
//         5..=8 => (speed * cph) as f64 * 0.9,
//         9..=10 => (speed * cph) as f64 * 0.77,
//         _ => 0 as f64,
//     }
// }

// pub fn working_items_per_minute(speed: u16) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }

//3.

## FILL in the blank to display the whole call stack
## Tips: you can find the clue in the default panic info 
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[97, 98, 99]`,
 right: `[96, 97, 98]`', src/main.rs:3:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:154:5
   4: study_cargo::main
             at ./src/main.rs:3:5
   5: core::ops::function::FnOnce::call_once
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
