// struct Array<T, const N: usize> {
//     data: [T; N],
// }
// fn main(){
//     let arrays = [
//         Array{data: [1, 2, 3]},
//         Array{data: [4, 5, 6]},
//         Array{data: [7, 8, 9]},
//         // Add more arrays as needed
//     ];
//     println!("successfully")
// }

//2

// fn print_array< T: std::fmt::Debug, const N:usize >( arr:[T; N] ) {
//     println!("{:?}", arr);
// }
// fn main(){
//     let arr = [1, 2, 3];
//     print_array(arr);
    
//     let arr = ["Hello", "World"];
//     print_array(arr);
// }

//3
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where 
//     Assert<{ core::mem::size_of::<T>()<768}>:IsTrue,
//     {
//         //...
//     }

//     fn main(){
//         check_size([0u8; 767]);
//         check_size([0i32; 191]);
//         check_size(["hello  "; 30 ]);  //size of &str
//         check_size([(); 31 ].map(|_| "hello  ".to_string())); //size of String?
//         check_size(['£'; 191 ]); //size of char?

//         println!("Success!");
//     }

//     pub enum Assert<const CHECK: bool>{}

//     pub trait IsTrue{}

//     impl IsTrue for Assert<true>{}

//3. Stable version

fn check_size<T>(val: T) {
    assert!(core::mem::size_of::<T>() < 768, "Type is too large!");
    //...
}

fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello"; 5]);  // Array of 5 &str references
    check_size([(); 1].map(|_| "hello".to_string())); // Array of 1 String
    check_size(['£'; 191]); // Array of 191 chars (4 bytes each)

    println!("Success!");
}
