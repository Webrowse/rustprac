/* Refactoring the following code using iterators */
// fn main() {
//     let arr = [0; 10];
//     for i in arr {
//         println!("{}",arr[i]);
//     }
// }

//2

// /* Fill in the blank */
// fn main() {
//     let mut v = Vec::new();
//     for n in 0..100 {
//        v.push(n);
//     }

//     assert_eq!(v.len(), 100);
// }

//3.

// /* Fill the blanks and fix the errors.
// Using two ways if possible */
// fn main() {
//     let mut v1 = vec![1, 2].into_iter();

//     assert_eq!(v1.next(), Some(1));
//     assert_eq!(v1.next(), Some(2));
//     assert_eq!(v1.next(), None);
// }


//4.

// /* Make it work */
// fn main() {
//     let arr = vec![0; 10];
//     for i in arr.iter() {
//         println!("{}", i);
//     }

//     println!("{:?}",arr);
// }

