#[derive(Debug, PartialEq)]
pub enum KvError {
    NoEquals,
    EmptyKey,
    BadValue(String),
}

/// Разбирает строку вида `key=value`, где value — `u32`.
///
/// # Errors
/// - [`KvError::NoEquals`] — нет символа `=`
/// - [`KvError::EmptyKey`] — ключ пустой
/// - [`KvError::BadValue`] — значение не парсится как `u32`
pub fn parse_kv(s: &str) -> Result<(&str, u32), KvError> {
    let Some((key, value)) = s.split_once('=') else {
        return Err(KvError::NoEquals);
    };
    if key.is_empty() {
        return Err(KvError::EmptyKey);
    }
    let n = value
        .parse::<u32>()
        .map_err(|_| KvError::BadValue(value.to_string()))?;
    Ok((key, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(parse_kv("x=1"), Ok(("x", 1)))
    }

    #[test]
    fn no_equals() {
        assert_eq!(parse_kv("x1"), Err(KvError::NoEquals))
    }

    #[test]
    fn empty_key() {
        assert_eq!(parse_kv("=1"), Err(KvError::EmptyKey))
    }

    #[test]
    fn empty_value() {
        assert_eq!(parse_kv("x="), Err(KvError::BadValue("".to_string())))
    }

    #[test]
    fn negative_value() {
        assert_eq!(parse_kv("x=-1"), Err(KvError::BadValue("-1".to_string())))
    }

    #[test]
    fn big_value() {
        assert_eq!(
            parse_kv("x=999999999999999999"),
            Err(KvError::BadValue("999999999999999999".to_string()))
        )
    }

    #[test]
    fn bad_value() {
        assert_eq!(
            parse_kv("speed=high"),
            Err(KvError::BadValue("high".to_string()))
        )
    }

    #[test]
    fn whitespaced_value() {
        assert_eq!(
            parse_kv(" speed = 42 "),
            Err(KvError::BadValue(" 42 ".to_string()))
        )
    }

    #[test]
    fn bad_value_two_equals() {
        assert_eq!(
            parse_kv("speed=x=f"),
            Err(KvError::BadValue("x=f".to_string()))
        )
    }
}
