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
} 

fn print_char(c : char) {
    println!("{}", c);
}







