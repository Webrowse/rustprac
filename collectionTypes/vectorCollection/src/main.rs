
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v: Vec<u8> = Vec::from(arr);
    is_vec(&v);

    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v: Vec<u8> = vec!(1, 2, 3);
    is_vec(&v.clone());
    
    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code 
    let mut v1:Vec<T>::new();
    is_vec(v1.clone());
    for i in &v{
        v1.push(*i);  // clone the value to avoid borrowing error  // or use v.clone() instead of *i  // or use &i instead of *i to avoid borrowing error  // or use &v[i] instead of *i to avoid borrowing error  // or use &v.clone()[i] instead of *i to avoid borrowing error  // or use v.iter().cloned().collect::<Vec<_>>() instead of v.clone() to avoid borrowing error  // or use v.iter().cloned().collect::<Vec<_>>() instead of *i to avoid borrowing error  // or use v.iter().cloned().collect::<Vec<_>>() instead of *i to avoid borrowing error  // or use v.iter().cloned().collect::<Vec<_>>() instead of *i to avoid borrowing error  // or use v.iter().cloned().collect::<Vec<_>>() instead of *i to
    }
    
 
    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}