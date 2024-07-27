
//1 function.
fn sum(x:i32, y: i32)->i32 {
    x + y
}
//2 function.
fn print() {
    println!("Success!2");
 }
//3.
// fn never_return() ->! {
//     todo!();
// }
//4.
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!();
    //  unimplement!()
    //  todo!()
}
fn main() {
//1. Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!1");
//2.

    print();

//3.
// never_return();

// println!("Failed!");

//4.
println!("Success!4");

//5.
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to 
        // replace a value of any value
        false => {
            println!("Success!5");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");










}




