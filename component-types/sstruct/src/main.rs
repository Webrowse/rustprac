
#[allow(unused_variables)]
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age, 
        hobby: String::from("eating sugar")
    };

    println!("Success!{}{}{}",p.name,p.age,p.hobby);

    
} 