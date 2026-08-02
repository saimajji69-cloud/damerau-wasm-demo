/// Calculates the Damerau-Levenshtein distance between two strings (handles adjacent transpositions).
pub fn damerau_levenshtein(string_1: &str, string_2: &str) -> usize {
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
    let mut dprev: Vec<usize> = d0.clone();

    for i in 0..len_1 {
        d1[0] = i + 1;
        for j in 0..len_2 {
            let mut cost = d0[j];

            if s1[i] != s2[j] {
                // substitution
                cost += 1;

                // insertion
                let x_cost = d1[j] + 1;
                if x_cost < cost {
                    cost = x_cost;
                }

                // deletion
                let y_cost = d0[j + 1] + 1;
                if y_cost < cost {
                    cost = y_cost;
                }

                // transposition
                if i > 0 && j > 0 && s1[i] == s2[j - 1] && s1[i - 1] == s2[j] {
                    let transp_cost = dprev[j - 1] + 1;
                    if transp_cost < cost {
                        cost = transp_cost;
                    }
                }
            }
            d1[j + 1] = cost;
        }

        let tmp_prev = dprev;
        dprev = d0;
        d0 = d1;
        d1 = tmp_prev;
    }

    d0[d0.len() - 1]
}
