// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     // Convert array to Vec<u8>
//     let v: Vec<u8> = Vec::from(arr);
//     is_vec(&v);

//     // Create a Vec<u8> directly
//     let v: Vec<u8> = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) is the same as vec![..]
//     let v: Vec<u8> = vec!(1, 2, 3);
//     is_vec(&v);  // Correctly pass a reference

//     // Using Vec::new and `for` to populate a Vec of arrays
//     let mut v1: Vec<[u8; 3]> = Vec::new();
//     v1.push(arr);

//     // Convert the first element (array) to Vec<u8> before passing to is_vec
//     is_vec(&v1[0].to_vec());

//     // Convert `v1[0]` to Vec<u8> to compare with `v`
//     assert_eq!(v, v1[0].to_vec());

//     println!("Success!");
// }

// // Function that accepts a reference to a Vec<u8>
// fn is_vec(v: &Vec<u8>) {}

//2.


// // FILL in the blank
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.extend(&v1);

//     assert_eq!(v1, v2);

//     println!("Success!");
// }

//3.


// // FILL in the blanks
// fn main() {
//     // Array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr:[i32; 3] = [1, 2, 3];
//     let v1: Vec<i32> = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
 
//     assert_eq!(v1, v2);
 
    
//     // String -> Vecinto
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // Iterators can be collected into vectors
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!");
//  }

//4. 
// Indexing 

// // FIX the error and IMPLEMENT the code
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i)) //Option<i32>
//     }

//     for i in 0..5 {
//        match v.get(i) {
//         Some(e) => v[i] = e + 1,
//         None => v.push(i + 2)
//        }
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }

//5.
//slicing


// // FIX the errors
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1: &[i32] = &v[..];
//     // Out of bounds will cause a panic
//     // You must use `v.len` here
//     let slice2: &[i32] = &v[0..v.len()];
    
//     assert_eq!(slice1, slice2);
    
//     // Slices are read only
//     // Note: slice and &Vec are different
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..];

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!");
// }

//6.
//Capacity

// // FIX the errors
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     // The vector contains no items, even though it has capacity for more
//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     // These are all done without reallocating...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     // ...but this may make the vector reallocate
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // Fill in an appropriate value to make the `for` done without reallocating 
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
    
//     println!("Success!");
// }

//7.
//

// #[derive(Debug,PartialEq)]

// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // FILL in the blank
//     let v : Vec<IpAddr>= vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//         // Add more IP addresses here if needed...  //Hint: Use a loop or a vector comprehension for this task.
//     ];
    
//     // Comparing two enums need to derive the PartialEq trait
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!");
// }

//8.

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    // FILL in the blank
    let v: Vec<Box<dyn IpAddr>>= vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}