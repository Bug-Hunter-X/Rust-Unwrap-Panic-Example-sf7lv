fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will panic if the vector is empty
    let first = vec.get(0).unwrap(); 
    println!("First element: {}", first);
}