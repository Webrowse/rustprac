
fn main() {
    // Fill the blank with proper array type
    let arr:[i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!1");

//2.
let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!2");

//3.
let list: [i32; 100] = [1; 100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!3");

//4.
let _arr = [1, 2, 3];

println!("Success!4");

//5.
let arr = ['a', 'b', 'c'];
    
let ele = arr[0]; // Only modify this line to make the code work!

assert!(ele == 'a');

println!("Success!5");

//6.
let names = [String::from("Sunfei"), "Sunface".to_string()];
    
// `Get` returns an Option<T>, it's safe to use
let name0 = names.get(0).unwrap();

// But indexing is not safe
let _name1 = &names[1];

println!("Success!6");



















}