#[warn(unused_imports)]
#[allow(unused_variables)]

fn main() {
    use std::mem::size_of_val;

    let c1 = 'a';
    println!("value of char: {}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");

//2.
// Make it work

    let c1 = '中'; //double quotes are string, single is char
    print_char(c1);


fn print_char(c : char) {
    println!("{}", c);
}

//3.
// Make println! work

    let _f: bool = false;
    
    let t = true;
    if t {
        println!("Success!");
    }

//4.
let f = true;
let t = true || false; // changed && to ||
assert_eq!(t, f);

println!("Success!");

//5.
let _v: () = ();

//let v = (2, 3);
assert_eq!(_v, implicitly_ret_unit());

println!("Success!");


fn implicitly_ret_unit() {
println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
println!("I will return a ()");

}
//6.

// Modify `4` in assert to make it work
//use std::mem::size_of_val;

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!4");

//7.

}

