//
// From a leetcode problem to tell the difference between two strings where
// there is a one character difference
//
fn main() {
    let s: String = String::from("Hello");
    let t: String = String::from("Hel#lo");

    // Figure out which has a greater length
    let mut length = 0;
    if s.len() > t.len() {
        length = s.len();
    } else {
        length = t.len();
    }

    let mut s_cum: u32 = 0;
    let mut t_cum: u32 = 0;

    // Loop through both strings get the cummulative of the byte
    // values of each string
    for i in 0..length {
        // Note: s.bytes().nth() is an Option so we need to check if there
        // is Some() before unwrapping otherwise our program will panic
        let s_char = s.bytes().nth(i);
        if s_char.is_some() {
            s_cum += u32::from(s.bytes().nth(i).unwrap());
        }

        let t_char = t.bytes().nth(i);
        if t_char.is_some() {
            t_cum += u32::from(t.bytes().nth(i).unwrap());
        }
    }

    // Since we know there is only a one character difference, we can simply
    // substract one from another and differece will be the missing character
    let mut diff_in_strs = 0;
    if (s_cum > t_cum) {
        diff_in_strs = (s_cum - t_cum) as u8;
    } else {
        diff_in_strs = (t_cum - s_cum) as u8;
    }

    println!("diff_in_strs={}", diff_in_strs as char);
}
