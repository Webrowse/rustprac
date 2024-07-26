//1.
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
       let _ = 2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

//2.
let v = {
    let mut x = 1;
    x += 2;
    x
};

assert_eq!(v, 3);

println!("Success!2");

//3
let v = {let x = 3; x};

assert!(v == 3);

println!("Success!3");

//4.
let s = sum(1 , 2);
assert_eq!(s, 3);

println!("Success!4");


fn sum(x: i32, y: i32) -> i32 {
x + y
}

}