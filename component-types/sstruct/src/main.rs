
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

// How can you believe sunface is only 18? 
p.age = 30;

// Fill the blank
p.name = String::from("sunfei");

println!("Success!{} and {}", p.name, p.age);

} 
fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }

