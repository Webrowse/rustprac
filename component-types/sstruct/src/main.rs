
#[allow(unused_variables)]
#[warn(unused_variables)]


// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}
impl SomeTrait for Unit {  }
fn do_something_with_unit(_u: Unit) { }
struct Color(i32, i32, i64);
struct Point(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let age:u8 = 30;
    let p: Person = Person {
        name: String::from("sunface"),
        age, 
        hobby: String::from("eating sugar"),
    };

    println!("Success!{}{}{}",p.name,p.age,p.hobby);
//3.
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!3");

//4.
let v:Point = Point(0, 127, 255);
check_color(v);

println!("Success!4");
//5.
let age = 18;
let mut p = Person {
    name: String::from("sunface"),
    age, 
    hobby: String::from("coding"),
};


p.age = 30;


p.name = String::from("sunfei");

println!("Success!{} and {}", p.name, p.age);
//6.
let u1: User = User {
    email: String::from("someone@example.com"),
    username: String::from("sunface"),
    active: true,
    sign_in_count: 1,
};
let u2 = set_email(u1);

println!("Success!6");

//7.
let scale = 2;
let rect1 = Rectangle {
    width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
    height: 50,
};

dbg!(&rect1); // Print debug info to stderr

println!("{:?}", rect1); // Print debug info to stdout

//8

let f = File {
    name: String::from("readme.md"),
    data: "Rust By Practice".to_string()
};

let _name = f.name.clone();

// ONLY modify this line
println!("{}, {}, {:?}",_name, f.data, f);
} 
fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }
 fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}