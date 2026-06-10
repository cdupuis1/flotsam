//
// Leetcode problem of seeing if two strings are anagrams.  An anagram is
// where two strings are the same length and use exactly the same letters
// though not necessary in the same order

// Prototype of the leetcode function
fn is_anagram(s: String, t: String) -> bool {
    // If the strings are not the same length then they
    // can't be anagrams
    if s.len() != t.len() {
        return false;
    }

    // In BTreeMaps the keys are sorted
    let mut s_counts: BTreeMap<char, u32> = BTreeMap::new();
    let mut t_counts: BTreeMap<char, u32> = BTreeMap::new();

    // Walk the strings and add the letter and count to the map
    for i in 0..s.len() {
        let c = s.chars().nth(i);
        let d = t.chars().nth(i);

        if c.is_some() {
            let s_count = s_counts.entry(c.unwrap()).or_insert(0);
            *s_count += 1;
        }
        if d.is_some() {
            let t_count = t_counts.entry(d.unwrap()).or_insert(0);
            *t_count += 1;
        }
    }

    // Now walk the BTreeMaps and see if there is a letter count that doesn't match
    println!("===s_values===");
    for (key, value) in &s_counts {
        println!("key={} value={}", key, value);
        match t_counts.get(key) {
            Some(t_key_count) => {
                // See if it's the same number of occurences
                if t_key_count != value {
                    println!("Keys match but different counts {} {}", value, t_key_count);
                    return false;
                }
            },
            None => {
                println!("No match for {}, return false", key);
                return false;
            }
        }
    }
 
    true
}

fn main() {
    let result = is_anagram("".to_string(), "".to_string());
    if result {
        println!("Strings match");
    } else {
        println!("Strings do no match");
    }
}
