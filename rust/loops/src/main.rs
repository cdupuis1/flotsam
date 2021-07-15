//
// Show samples of various for loops
//
fn main() {
    // Create mutable vector
    let mut all: Vec<i32> = Vec::new();

    // Add some elements to it
    all.push(1);
    all.push(2);
    all.push(3);
    all.push(4);

    // Iterate over the contents in a foreach type fashion
    for a in all.iter() {
        println!("a={}", a);
    }

    // Iterate over the vector in a more classic array fashion
    for i in 0..all.len() {
        println!("all[{}]={}", i, all[i]);
    }

    // Now enumerate the vector using a (index, element) tuple
    for (i, a) in all.iter().enumerate() {
        println!("all[{}]={}", i, a);
    }
}
