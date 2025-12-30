fn substring(haystack: String, needle: String) -> i32 {
    let mut pos_found: i32 = -1;

    if needle.len() != 0 &&
        haystack.len() != 0 &&
        haystack.len() >= needle.len() {
        
        // Collect chars once to avoid repeated nth() calls
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        
        // Only check positions where needle could fit
        for start_pos in 0..=(haystack_chars.len() - needle_chars.len()) {
            let mut found = true;
            for substr_pos in 0..needle_chars.len() {
                println!("start_pos={} substr_pos={} haystack={} needle={}",
                    start_pos,
                    substr_pos,
                    haystack_chars[start_pos + substr_pos],
                    needle_chars[substr_pos],
                );
                if needle_chars[substr_pos] != haystack_chars[start_pos + substr_pos] {
                    found = false;
                    println!("String not found");
                    break;
                }
            }

            if found == true {
                pos_found = start_pos as i32;
                break;
            }
        }
    }

    pos_found
}

fn main() {
    let string1: String = String::from("abc");
    let substr: String = String::from("bc");

    let found = substring(string1, substr);
    println!("found={}", found);
}
