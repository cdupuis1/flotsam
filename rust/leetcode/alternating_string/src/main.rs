//
// Combines two strings by alternating one character from each string. Was part
// of a leetcode challenge
//
fn main() {
    let string1: String = String::from("hello");
    let string2: String = String::from("world!");
    let mut result_string: String = String::from("");
    let mut num_iters = 0;

    if string1.len() > string2.len() {
        num_iters = string1.len();
    } else {
        num_iters = string2.len();
    }

    for i in 0..num_iters {
        let c1 = string1.chars().nth(i);
        let c2 = string2.chars().nth(i);
        if c1.is_some() {
            result_string.push(c1.unwrap());
        }
        if c2.is_some() {
            result_string.push(c2.unwrap());
        }
    }

    println!("result_string=\"{}\"", result_string);
    ()
}
