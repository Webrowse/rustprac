// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) ->String;
// }

// struct Student {}
// impl Hello for Student {
    
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) ->String{
//         String::from("hi, I'm your new teacher")
//     }
//     fn say_something(&self) ->String{
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main(){
//     let s = Student{};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher{};
//     assert_eq!(t.say_hi(), "hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!");
// }

//2.
//Derive

// // `Centimeters`, a tuple struct that can be compared
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// // `Inches`, a tuple struct that can be printed
// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches):&Inches = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// // ADD some attributes to make the code work!
// // DON'T modify other code!
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second:Seconds = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_false = _one_second > _one_second;

//     let foot: Inches = Inches(12);

//     println!("One foot equals {:?}", foot);

//     let meter: Centimeters = Centimeters(100.0);

//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };

//     println!("One foot is {} than one meter.", cmp);
// }


//3.
//Operator

// // Implement fn multiply to make the code work.
// // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// // So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
// fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
//     a * b   //a.mul(b)
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));

//     println!("Success!");
// }

//4.

// Fix the errors, DON'T modify the code in `main`.
use std::ops;

struct Foo;
struct Bar;
#[derive(Debug, PartialEq)]
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

fn main() {
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}