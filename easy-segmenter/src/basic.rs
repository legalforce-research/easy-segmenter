use regex::Regex;

///
pub fn in_periods() -> &'static [&'static str] {
    &["。", "．", "？", "！", "? ", "! "]
}

///
pub fn ex_periods() -> &'static [&'static str] {
    &["\n", "\r\n", "\r"]
}

///
pub fn parentheses() -> &'static [(char, char)] {
    &[
        ('(', ')'),
        ('[', ']'),
        ('（', '）'),
        ('「', '」'),
        ('【', '】'),
        ('『', '』'),
        ('［', '］'),
        ('〔', '〕'),
    ]
}

pub fn decimal_point() -> Regex {
    Regex::new(r"\d(．)\d").unwrap()
}
