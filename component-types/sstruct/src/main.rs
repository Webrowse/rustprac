
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
fn do_something_with_unit(u: Unit) { }
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age, 
        hobby: String::from("eating sugar")
    };

    println!("Success!{}{}{}",p.name,p.age,p.hobby);
//3.
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!3");

//4.
let v = Point(__, __, __);
check_color(v);

println!("Success!");
} 
fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
 }

