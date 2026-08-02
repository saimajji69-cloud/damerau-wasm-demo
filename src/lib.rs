pub mod classic;
pub mod damerau;
pub mod recursive;
pub mod wf;

pub use classic::classic_levenshtein;
pub use damerau::damerau_levenshtein;
pub use recursive::recursive_levenshtein;
pub use wf::{wf_levenshtein, wfi_levenshtein};



/// Alias for `wfi_levenshtein`, matching Python `pylev.levenshtein`.
pub use wfi_levenshtein as levenshtein;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn distance(a: &str, b: &str) -> usize {
    damerau_levenshtein(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damerau_levenshtein() {
        assert_eq!(damerau_levenshtein("ba", "abc"), 2);
        assert_eq!(damerau_levenshtein("foobar", "foobra"), 1);
        assert_eq!(damerau_levenshtein("fee", "deed"), 2);
        assert_eq!(damerau_levenshtein("kitten", "sitting"), 3);
        assert_eq!(damerau_levenshtein("kitten", "kittne"), 1);
        assert_eq!(damerau_levenshtein("", ""), 0);
    }

    #[test]
    fn test_levenshtein_algorithms() {
        let test_cases = vec![
            ("kitten", "sitting", 3),
            ("kitten", "kitten", 0),
            ("", "", 0),
            ("meilenstein", "levenshtein", 4),
            ("levenshtein", "frankenstein", 6),
            ("confide", "deceit", 6),
            ("CUNsperrICY", "conspiracy", 8),
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(recursive_levenshtein(a, b), expected, "recursive failed for {}, {}", a, b);
            assert_eq!(recursive_levenshtein(b, a), expected, "recursive failed for {}, {}", b, a);

            assert_eq!(wf_levenshtein(a, b), expected, "wf failed for {}, {}", a, b);
            assert_eq!(wf_levenshtein(b, a), expected, "wf failed for {}, {}", b, a);

            assert_eq!(wfi_levenshtein(a, b), expected, "wfi failed for {}, {}", a, b);
            assert_eq!(wfi_levenshtein(b, a), expected, "wfi failed for {}, {}", b, a);

            assert_eq!(levenshtein(a, b), expected, "levenshtein alias failed for {}, {}", a, b);
            assert_eq!(levenshtein(b, a), expected, "levenshtein alias failed for {}, {}", b, a);
        }
    }

    #[test]
    fn test_classic_levenshtein() {
        assert_eq!(classic_levenshtein("kitten", "sitting"), 3);
        assert_eq!(classic_levenshtein("kitten", "kitten"), 0);
        assert_eq!(classic_levenshtein("", ""), 0);
        assert_eq!(classic_levenshtein("test", "test"), 0);
        assert_eq!(classic_levenshtein("", "test"), 4);
        assert_eq!(classic_levenshtein("test", ""), 4);
    }
}
