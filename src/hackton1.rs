/// Calculates the Levenshtein distance using the Wagner-Fischer algorithm (1D matrix).
pub fn wf_levenshtein(string_1: &str, string_2: &str) -> usize {
    let s1: Vec<char> = string_1.chars().collect();
    let s2: Vec<char> = string_2.chars().collect();

    let len_1 = s1.len() + 1;
    let len_2 = s2.len() + 1;

    let mut d = vec![0usize; len_1 * len_2];

    for i in 0..len_1 {
        d[i] = i;
    }
    for j in 0..len_2 {
        d[j * len_1] = j;
    }

    for j in 1..len_2 {
        for i in 1..len_1 {
            if s1[i - 1] == s2[j - 1] {
                d[i + j * len_1] = d[i - 1 + (j - 1) * len_1];
            } else {
                let del = d[i - 1 + j * len_1] + 1;
                let ins = d[i + (j - 1) * len_1] + 1;
                let sub = d[i - 1 + (j - 1) * len_1] + 1;
                d[i + j * len_1] = del.min(ins).min(sub);
            }
        }
    }

    d[d.len() - 1]
}

/// Calculates the Levenshtein distance using the iterative Wagner-Fischer algorithm (two rows).
pub fn wfi_levenshtein(string_1: &str, string_2: &str) -> usize {
    if string_1 == string_2 {
        return 0;
    }

    let mut s1: Vec<char> = string_1.chars().collect();
    let mut s2: Vec<char> = string_2.chars().collect();

    let mut len_1 = s1.len();
    let mut len_2 = s2.len();

    if len_1 == 0 {
        return len_2;
    }
    if len_2 == 0 {
        return len_1;
    }

    if len_1 > len_2 {
        std::mem::swap(&mut s1, &mut s2);
        std::mem::swap(&mut len_1, &mut len_2);
    }

    let mut d0: Vec<usize> = (0..=len_2).collect();
    let mut d1: Vec<usize> = (0..=len_2).collect();

    for i in 0..len_1 {
        d1[0] = i + 1;
        for j in 0..len_2 {
            let mut cost = d0[j];

            if s1[i] != s2[j] {
                cost += 1;

                let x_cost = d1[j] + 1;
                if x_cost < cost {
                    cost = x_cost;
                }

                let y_cost = d0[j + 1] + 1;
                if y_cost < cost {
                    cost = y_cost;
                }
            }

            d1[j + 1] = cost;
        }

        std::mem::swap(&mut d0, &mut d1);
    }

    d0[d0.len() - 1]
}
