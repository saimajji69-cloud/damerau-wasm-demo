use std::collections::HashMap;

fn recursive_helper(
    s1: &[char],
    s2: &[char],
    len_1: usize,
    len_2: usize,
    offset_1: usize,
    offset_2: usize,
    memo: &mut HashMap<(usize, usize, usize, usize), usize>,
) -> usize {
    let key = (offset_1, len_1, offset_2, len_2);
    if let Some(&val) = memo.get(&key) {
        return val;
    }

    if len_1 == 0 {
        return len_2;
    } else if len_2 == 0 {
        return len_1;
    }

    let cost = if s1[offset_1] != s2[offset_2] { 1 } else { 0 };

    let d1 = recursive_helper(s1, s2, len_1 - 1, len_2, offset_1 + 1, offset_2, memo) + 1;
    let d2 = recursive_helper(s1, s2, len_1, len_2 - 1, offset_1, offset_2 + 1, memo) + 1;
    let d3 = recursive_helper(s1, s2, len_1 - 1, len_2 - 1, offset_1 + 1, offset_2 + 1, memo) + cost;

    let dist = d1.min(d2).min(d3);
    memo.insert(key, dist);
    dist
}

/// Calculates the Levenshtein distance between two strings using memoized recursion.
pub fn recursive_levenshtein(string_1: &str, string_2: &str) -> usize {
    let s1: Vec<char> = string_1.chars().collect();
    let s2: Vec<char> = string_2.chars().collect();
    let mut memo = HashMap::new();
    recursive_helper(&s1, &s2, s1.len(), s2.len(), 0, 0, &mut memo)
}
