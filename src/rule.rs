use std::collections::BTreeMap;

use serde_derive::Deserialize;

use crate::errors::Result;

/// Configure of segmentation rules.
#[derive(Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(default)]
struct RuleConfig {
    in_delimiters: Vec<String>,
    ex_delimiters: Vec<String>,
    quotes: Vec<String>,
    words: Vec<String>,
    regex: BTreeMap<String, String>,
}

impl RuleConfig {
    /// Deserializes a string in the TOML format into a [`RuleConfig`].
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
    /// # Errors
    ///
    /// [`toml::de::Error`] will be reported if the deserialization fails.
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

        let rule_set = RuleConfig::from_toml_str(toml_str).unwrap();
        let expected = RuleConfig {
            in_delimiters: vec!["。".to_string()],
            ex_delimiters: vec!["\n".to_string(), "\r\n".to_string(), "\r".to_string()],
            quotes: vec![],
            words: vec![],
            regex: BTreeMap::new(),
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

        let rule_set = RuleConfig::from_toml_str(toml_str).unwrap();
        let expected = RuleConfig {
            in_delimiters: vec!["。".to_string(), "．".to_string()],
            ex_delimiters: vec!["\n".to_string(), "\r\n".to_string(), "\r".to_string()],
            quotes: vec!["「」".to_string(), "（）".to_string()],
            words: vec!["モーニング娘。".to_string()],
            regex: BTreeMap::from([
                ("decimal_point".to_string(), r"\d(．)\d".to_string()),
                ("dot_sequence".to_string(), r"(。{2,})。".to_string()),
            ]),
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_empty_members() {
        let toml_str = "";

        let rule_set = RuleConfig::from_toml_str(toml_str).unwrap();
        let expected = RuleConfig {
            in_delimiters: vec![],
            ex_delimiters: vec![],
            quotes: vec![],
            words: vec![],
            regex: BTreeMap::new(),
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_undefined_member() {
        let toml_str = r#"
            in_delimiters = ["。"]
            out_delimiters = ["\n"] # will be ignored
        "#;
        let rule_set = RuleConfig::from_toml_str(toml_str).unwrap();
        let expected = RuleConfig {
            in_delimiters: vec!["。".to_string()],
            ex_delimiters: vec![],
            quotes: vec![],
            words: vec![],
            regex: BTreeMap::new(),
        };
        assert_eq!(rule_set, expected);
    }

    #[test]
    fn test_from_toml_str_broken_format() {
        let toml_str = r#"
            in_delimiters = ["。"
        "#;
        assert!(RuleConfig::from_toml_str(toml_str).is_err());
    }
}
