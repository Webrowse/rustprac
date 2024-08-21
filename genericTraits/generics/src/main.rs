
// // Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('a'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen('b'));

//     println!("Success!");
// }

//2.


// // Implement the generic function below.
// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) ->T{
//     a+b
// } 

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }

//3.


// // Implement struct Point to make it work.
// struct Point<T> {
//     x:T,
//     y:T,
    
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };

//     println!("Success!");

// }

//4.


// // Modify this struct to make the code work
// struct Point<T, S> {
//     x: T,
//     y: S,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }

//5.


// // Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }


// fn main() {
//     let x: Val<f64> = Val{ val: 3.0 };
//     let y: Val<String> = Val{ val: "hello".to_string()};
//     println!("{}, {:?}", x.value(), y.value());
// }

//6.

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<V, W>(self, romy:Point<V, W>) -> Point<T,W> {
//         Point{
//             x: self.x,
//             y: romy.y,
//         }
//     }
// }

// fn main() {
//     let p1: Point<i32, i32> = Point { x: 5, y: 10 };
//     let p2: Point<&str, char> = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }

//7.


// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p:Point<f64> = Point{x: 5f64, y: 10f64};
    println!("{}",p.distance_from_origin());
}