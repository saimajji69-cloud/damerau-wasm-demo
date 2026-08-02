fn classic_levenshtein_chars(s1: &[char], s2: &[char]) -> usize {
    let len_1 = s1.len();
    let len_2 = s2.len();
    let cost = if len_1 > 0 && len_2 > 0 && s1[0] != s2[0] { 1 } else { 0 };

    if len_1 == 0 {
        len_2
    } else if len_2 == 0 {
        len_1
    } else {
        let d1 = classic_levenshtein_chars(&s1[1..], s2) + 1;
        let d2 = classic_levenshtein_chars(s1, &s2[1..]) + 1;
        let d3 = classic_levenshtein_chars(&s1[1..], &s2[1..]) + cost;
        d1.min(d2).min(d3)
    }
}

/// Calculates the Levenshtein distance between two strings using classic recursion.
pub fn classic_levenshtein(string_1: &str, string_2: &str) -> usize {
    let s1: Vec<char> = string_1.chars().collect();
    let s2: Vec<char> = string_2.chars().collect();
    classic_levenshtein_chars(&s1, &s2)
}
