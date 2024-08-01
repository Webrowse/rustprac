
fn greetings(s: &str) {
    println!("{}",s)
}
fn main() {
    let s: &str = "hello, world";

    println!("Success!1");

//2.
    let s: Box<str> = "hello, world".into();
    greetings(&s)



}