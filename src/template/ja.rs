//! Basic segmentation rules in Japanese.
use regex::Regex;

/// Creates a list of basic inclusive delimiters.
///
/// See the source code for the specific definition.
///
/// # Examples
///
/// ```
/// use easy_segmenter::{template, SegmenterBuilder};
///
/// let seg = SegmenterBuilder::new()
///     .in_delimiters(template::ja::in_delimiters())
///     .build()
///     .unwrap();
/// let text = "それは何ですか？ペンです。";
/// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
/// let expected = vec!["それは何ですか？", "ペンです。"];
/// assert_eq!(sentences, expected);
/// ```
pub fn in_delimiters() -> Vec<&'static str> {
    vec!["。", "．", "？", "！", "?", "!"]
}

/// Creates a list of basic exclusive delimiters.
///
/// See the source code for the specific definition.
///
/// # Examples
///
/// ```
/// use easy_segmenter::{template, SegmenterBuilder};
///
/// let seg = SegmenterBuilder::new()
///     .ex_delimiters(template::ja::ex_delimiters())
///     .build()
///     .unwrap();
/// let text = "これはペンです\r\nそれはマーカーです\n";
/// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
/// let expected = vec!["これはペンです", "それはマーカーです"];
/// assert_eq!(sentences, expected);
/// ```
pub fn ex_delimiters() -> Vec<&'static str> {
    vec!["\n", "\r\n", "\r"]
}

/// Creates a list of parentheses for quoatations.
///
/// See the source code for the specific definition.
///
/// # Examples
///
/// ```
/// use easy_segmenter::{template, SegmenterBuilder};
///
/// let seg = SegmenterBuilder::new()
///     .in_delimiters(template::ja::in_delimiters())
///     .parentheses(template::ja::parentheses())
///     .build()
///     .unwrap();
/// let text = "私は「はい。そうです。」と答えた。";
/// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
/// let expected = vec!["私は「はい。そうです。」と答えた。"];
/// assert_eq!(sentences, expected);
/// ```
pub fn parentheses() -> Vec<(char, char)> {
    vec![
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

/// Creates a regex to recognize decimal points.
///
/// See the source code for the specific definition.
///
/// # Examples
///
/// ```
/// use easy_segmenter::{template, SegmenterBuilder};
///
/// let seg = SegmenterBuilder::new()
///     .in_delimiters(["．"])
///     .no_break_regex(template::ja::decimal_point())
///     .build()
///     .unwrap();
/// let text = "三．一四";
/// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
/// let expected = vec!["三．一四"];
/// assert_eq!(sentences, expected);
/// ```
pub fn decimal_point() -> Regex {
    let numbers = r"[0-9０-９〇零一二三四五六七八九十百千万億兆]";
    Regex::new(&format!(r"{numbers}([．.]){numbers}")).unwrap()
}
