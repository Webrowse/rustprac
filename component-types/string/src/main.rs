
fn greetings(s: &str) {
    println!("{}",s)
}
fn greetings7(s: String) {
    println!("{}", s)
}
fn main() {
    let s: &str = "hello, world";

    println!("Success!1");

//2.
    let s: Box<str> = "hello, world2".into();
    greetings(&s);

//3.
let mut s = String::from("");
s.push_str("hello, world");
s.push('!');

assert_eq!(s, "hello, world!");

println!("Success!3");

//4.
let mut s = String::from("hello");
s.push(',');
s.push_str(" world4");
s += "!";

println!("{}", s);

//5.
let s = String::from("I like dogs");
// Allocate new memory and store the modified string there
let s1 = s.replace("dogs", "cats");

assert_eq!(s1, "I like cats");

println!("Success!5");

//6.
let s1 = String::from("hello,");
    let s2 = "world!";
    let s3 = s1.clone() + s2; // s2.as_str() to convert string to stringslice
    // &s2 will also make &String -> &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);


}