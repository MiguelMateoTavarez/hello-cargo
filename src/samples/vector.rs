fn main() {
    //* Vector */
    let three_numbers = vec![15, 2, 46];
    let zeroes: Vec<i32> = vec![0; 5];
    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Initial vector: {:?}", three_numbers);
    println!("Zeroes: {:?}", zeroes);
    println!("Fruits: {:?}", fruit);

    //Remove final value of a vector
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let index_vec = vec![15, 3, 46];
    println!("Vector: {:?}", index_vec);
    //Access by index
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
}
