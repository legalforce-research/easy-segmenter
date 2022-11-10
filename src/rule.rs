use std::collections::BTreeMap;

use serde_derive::Deserialize;

use crate::errors::Result;

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct RuleSet {
    in_delimiters: Option<Vec<String>>,
    ex_delimiters: Option<Vec<String>>,
    quotes: Option<Vec<String>>,
    words: Option<Vec<String>>,
    regex: Option<BTreeMap<String, String>>,
}

impl RuleSet {
    /// Deserializes a string in the TOML format into a [`RuleSet`].
    ///
    /// # Format
    ///
    /// ```toml
    /// in_delimiters = ["。", "．"]
    /// ex_delimiters = ["\n", "\r\n", "\r"]
    /// quotes = ["「」", "（）"]
    /// words = ["モーニング娘。"]
    /// [regex]
    /// decimal_point = '\d(．)\d'
    /// dot_sequence = '(。{2,})。'
    /// ```
    ///
    /// ## The `in_delimiters` field
    ///
    /// # Errors
    pub fn from_toml_str<S>(toml_str: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        Ok(toml::from_str(toml_str.as_ref())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_toml_str_some_members() {
        let toml_str = r#"
            in_delimiters = ["。"]
            ex_delimiters = ["\n", "\r\n", "\r"]
        "#;

        let rule_set = RuleSet::from_toml_str(toml_str).unwrap();
        let expected = RuleSet {
            in_delimiters: Some(vec!["。".to_string()]),
            ex_delimiters: Some(vec!["\n".to_string(), "\r\n".to_string(), "\r".to_string()]),
            quotes: None,
            words: None,
            regex: None,
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_full_members() {
        let toml_str = r#"
            in_delimiters = ["。", "．"]
            ex_delimiters = ["\n", "\r\n", "\r"]
            quotes = ["「」", "（）"]
            words = ["モーニング娘。"]
            [regex]
            decimal_point = '\d(．)\d'
            dot_sequence = '(。{2,})。'
        "#;

        let rule_set = RuleSet::from_toml_str(toml_str).unwrap();
        let expected = RuleSet {
            in_delimiters: Some(vec!["。".to_string(), "．".to_string()]),
            ex_delimiters: Some(vec!["\n".to_string(), "\r\n".to_string(), "\r".to_string()]),
            quotes: Some(vec!["「」".to_string(), "（）".to_string()]),
            words: Some(vec!["モーニング娘。".to_string()]),
            regex: Some(BTreeMap::from([
                ("decimal_point".to_string(), r"\d(．)\d".to_string()),
                ("dot_sequence".to_string(), r"(。{2,})。".to_string()),
            ])),
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_empty_members() {
        let toml_str = "";

        let rule_set = RuleSet::from_toml_str(toml_str).unwrap();
        let expected = RuleSet {
            in_delimiters: None,
            ex_delimiters: None,
            quotes: None,
            words: None,
            regex: None,
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_undefined_member() {
        let toml_str = r#"
            in_delimiters = ["。"]
            out_delimiters = ["\n"] # will be ignored
        "#;
        let rule_set = RuleSet::from_toml_str(toml_str).unwrap();
        let expected = RuleSet {
            in_delimiters: Some(vec!["。".to_string()]),
            ex_delimiters: None,
            quotes: None,
            words: None,
            regex: None,
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_broken_format() {
        let toml_str = r#"
            in_delimiters = ["。"
        "#;
        assert!(RuleSet::from_toml_str(toml_str).is_err());
    }
}
