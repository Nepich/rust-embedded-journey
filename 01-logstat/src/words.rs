use std::collections::HashMap;

/// Разбираем строку, чистим от пробельных символов,
/// считаем частоту вхождения одинаковых слов и возвращаем n самых частотных.
/// В случае если n больше количества слов возвращаем все
pub fn top_words(text: &str, n: usize) -> Vec<(&str, u32)> {
    let words = text.split_whitespace();
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for n in words {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mut v: Vec<(&str, u32)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    v.truncate(n);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            top_words("a a a a a b b c c c d", 2),
            vec![("a", 5), ("c", 3)]
        )
    }

    #[test]
    fn too_big_n() {
        assert_eq!(top_words("a a b", 3), vec![("a", 2), ("b", 1)])
    }

    #[test]
    fn empty_string() {
        assert_eq!(top_words("", 1), vec![])
    }

    #[test]
    fn equal_frequency() {
        assert_eq!(top_words("a a b  c c d d", 2), vec![("a", 2), ("c", 2)])
    }
}
