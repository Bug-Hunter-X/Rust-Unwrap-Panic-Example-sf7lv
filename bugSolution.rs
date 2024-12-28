fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to handle potential empty vector
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty"),
    };
    //Alternative using if let
    if let Some(first) = vec.get(0) {
        println!("First element: {}", first);
    } else {
        println!("Vector is empty");
    }
} 