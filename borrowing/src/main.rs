fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
 
    //2.

    let x = 5;
    let _y = &x;

    // Modify this line only
    assert_eq!(5, x);

    println!("Success!2");

//3.
let mut s = String::from("hello, ");

borrow_object(&s);

println!("Success!3");

//4.


}
//3.
fn borrow_object(s: &String) {}


