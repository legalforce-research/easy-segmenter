pub mod builder;
pub use builder::SegmenterBuilder;

use regex::Regex;

use crate::basic;
use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};

const DEFAULT_MAX_QUOTE_LEVEL: usize = 3;

/// Simple rule-based segmenter.
pub struct Segmenter {
    // Breakers
    period_matcher: PeriodMatcher,
    // Non Breakers
    quote_matcher: QuoteMatcher,
    word_matcher: WordMatcher,
    regex_matchers: Vec<Regex>,
    max_quote_level: usize,
}

impl Segmenter {
    fn new(
        period_matcher: PeriodMatcher,
        quote_matcher: QuoteMatcher,
        word_matcher: WordMatcher,
        regex_matchers: Vec<Regex>,
    ) -> Self {
        Self {
            period_matcher,
            quote_matcher,
            word_matcher,
            regex_matchers,
            max_quote_level: DEFAULT_MAX_QUOTE_LEVEL,
        }
    }

    /// Creates an instance with basic segmentation rules.
    pub fn basic_configure() -> Self {
        SegmenterBuilder::new()
            .in_periods(basic::in_periods())
            .ex_periods(basic::ex_periods())
            .parentheses(basic::parentheses())
            .no_break_regex(basic::decimal_point())
            .build()
    }

    /// Segments an input text into sentences, returning byte-position ranges.
    pub fn segment<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut no_break = vec![false; text.len()];

        // TODO: Parallelization
        self.find_quotes(text, &mut no_break);
        self.find_words(text, &mut no_break);
        self.find_regex(text, &mut no_break);

        let mut start_pos = 0;

        self.period_matcher.iter(text).filter_map(move |m| {
            // Handling the last imaginary terminator.
            if m.start == m.end {
                if start_pos < text.len() {
                    // The case that the last character does not have any period.
                    return Some((start_pos, text.len()));
                } else {
                    return None;
                }
            }
            // if is_in_period, the period should be inclusive in the segment;
            // otherwise, the period should be exclusive in the segment.
            let end_pos = if m.is_in_period { m.end } else { m.start };
            if end_pos != 0 && no_break[end_pos - 1] {
                None
            } else {
                let range = (start_pos, end_pos);
                start_pos = m.end;
                Some(range)
            }
        })
    }

    fn find_quotes(&self, text: &str, detected: &mut [bool]) {
        let mut stack = vec![];
        for m in self.quote_matcher.iter(text) {
            if m.is_open {
                stack.push((m.start, m.id));
                continue;
            }
            if stack.is_empty() {
                continue;
            }
            let (start, id) = stack.last().cloned().unwrap();
            if id != m.id {
                continue; // No correspondence.
            }
            // NOTE: Since nested quates are processed, this algorithm runs in
            // O(nk) time, where n is text.len() and k is the max nesting level.
            if stack.len() <= self.max_quote_level {
                for b in detected.iter_mut().take(m.end).skip(start) {
                    *b = true;
                }
            }
            stack.pop();
        }
    }

    fn find_words(&self, text: &str, detected: &mut [bool]) {
        for m in self.word_matcher.iter(text) {
            for b in detected.iter_mut().take(m.end).skip(m.start) {
                *b = true;
            }
        }
    }

    fn find_regex(&self, text: &str, detected: &mut [bool]) {
        for re in &self.regex_matchers {
            for cap in re.captures_iter(text) {
                for idx in 1..cap.len() {
                    if let Some(m) = cap.get(idx) {
                        for i in m.range() {
                            detected[i] = true;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_1() {
        let seg = SegmenterBuilder::new().in_periods(["。"]).build();
        let text = "これはペンです。それはマーカーです。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["これはペンです。", "それはマーカーです。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_2() {
        let seg = SegmenterBuilder::new().in_periods(["。", "？"]).build();
        let text = "それは何ですか？ペンです。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["それは何ですか？", "ペンです。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_3() {
        let seg = SegmenterBuilder::new()
            .in_periods(["！"])
            .ex_periods(["\n"])
            .build();
        let text = "良かったね\nすごい！";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["良かったね", "すごい！"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_4() {
        let seg = SegmenterBuilder::new().ex_periods(["\n", "</br>"]).build();
        let text = "良かったね</br>すごい\n";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["良かったね", "すごい"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_duplicate_periods() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。", "。。。"])
            .ex_periods(["\n", "\r\n", "\r"])
            .build();
        let text = "なるほど。。。その通りですね\r\n";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["なるほど。。。", "その通りですね"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_listing() {
        let seg = SegmenterBuilder::new().ex_periods(["\n", "\n・"]).build();
        let text = "買うもの\n・ご飯\n・卵\n・醤油";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["買うもの", "ご飯", "卵", "醤油"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_1() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .parentheses([('「', '」')])
            .build();
        let text = "私は「はい。そうです。」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。そうです。」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_2() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .parentheses([('（', '）')])
            .build();
        let text = "私は「はい。そうです。」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。", "そうです。", "」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_3() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .parentheses([('「', '」'), ('（', '）')])
            .build();
        let text = "私は「はい。そうです。（嘘だけど。）」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。そうです。（嘘だけど。）」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_4() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .parentheses([('「', '」'), ('（', '）')])
            .build();
        let text = "私は「はい。そうです。（嘘だけど。）と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。", "そうです。", "（嘘だけど。）と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_word_1() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .no_break_words(["モーニング娘。"])
            .build();
        let text = "モーニング娘。の新曲。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["モーニング娘。の新曲。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_regex_1() {
        let seg = SegmenterBuilder::new()
            .in_periods(["．"])
            .no_break_regex(Regex::new(r"\d(．)\d").unwrap())
            .build();
        let text = "３．１４１５９２．円周率です．";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["３．１４１５９２．", "円周率です．"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_regex_2() {
        let seg = SegmenterBuilder::new()
            .in_periods(["。"])
            .no_break_regex(Regex::new(r"(。{2,})。").unwrap())
            .build();
        let text = "はぁ。。。疲れた。。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["はぁ。。。", "疲れた。", "。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_no_last_period() {
        let seg = SegmenterBuilder::new().in_periods(["。"]).build();
        let text = "これはペンです。それはマーカーです";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["これはペンです。", "それはマーカーです"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_empty_text() {
        let seg = SegmenterBuilder::new().in_periods(["。"]).build();
        let text = "";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        assert!(sentences.is_empty());
    }

    #[test]
    fn test_basic() {
        let seg = SegmenterBuilder::new()
            .in_periods(crate::basic::in_periods())
            .build();
        let text = "これはペンです。それはマーカーです。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["これはペンです。", "それはマーカーです。"];
        assert_eq!(sentences, expected);
    }
}
