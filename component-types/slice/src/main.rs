
#[allow(unused_variables)]
// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;

    println!("Success!1");
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, 
    //if it is an array, then `assert!` will be passed: 
    //Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!2");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, [2, 3, 4]);

    println!("Success!3");

    //4
    let s: String = String::from("hello");

    let slice1:&str = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!4");
    //5.
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!5");

    //6.
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter = first_letter(& s);
    println!("the first letter is: {}6", letter);

    s.clear(); // error!



}
fn first_letter(s: &str) -> &str {
    &s[..1]
}