fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    // Convert array to Vec<u8>
    let v: Vec<u8> = Vec::from(arr);
    is_vec(&v);

    // Create a Vec<u8> directly
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) is the same as vec![..]
    let v: Vec<u8> = vec!(1, 2, 3);
    is_vec(&v);  // Correctly pass a reference

    // Using Vec::new and `for` to populate a Vec of arrays
    let mut v1: Vec<[u8; 3]> = Vec::new();
    v1.push(arr);

    // Convert the first element (array) to Vec<u8> before passing to is_vec
    is_vec(&v1[0].to_vec());

    // Convert `v1[0]` to Vec<u8> to compare with `v`
    assert_eq!(v, v1[0].to_vec());

    println!("Success!");
}

// Function that accepts a reference to a Vec<u8>
fn is_vec(v: &Vec<u8>) {}

