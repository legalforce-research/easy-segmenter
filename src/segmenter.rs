pub mod builder;
pub use builder::JaSegmenterBuilder;

use regex::Regex;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};

pub struct JaSegmenter {
    period_matcher: PeriodMatcher,
    quote_matcher: QuoteMatcher,
    word_matcher: WordMatcher,
    regex_matchers: Vec<Regex>,
}

impl JaSegmenter {
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
        }
    }

    pub fn segment<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut no_break = vec![false; text.len()];
        self.find_quotes(text, &mut no_break);
        self.find_words(text, &mut no_break);
        self.find_regex(text, &mut no_break);

        let mut start_pos = 0;
        self.period_matcher.iter(text).filter_map(move |m| {
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
        let mut level = 0;
        let mut open_start = 0;
        for m in self.quote_matcher.iter(text) {
            if m.is_open {
                if level == 0 {
                    open_start = m.start;
                }
                level += 1;
            } else if level > 0 {
                level -= 1;
                if level == 0 {
                    for i in open_start..m.end {
                        detected[i] = true;
                    }
                }
            }
        }
    }

    fn find_words(&self, text: &str, detected: &mut [bool]) {
        for m in self.word_matcher.iter(text) {
            for i in m.start..m.end {
                detected[i] = true;
            }
        }
    }

    fn find_regex(&self, text: &str, detected: &mut [bool]) {
        for re in &self.regex_matchers {
            for m in re.find_iter(text) {
                for i in m.range() {
                    detected[i] = true;
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
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .build();
        let text = "これはペンです。それはマーカーです。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["これはペンです。", "それはマーカーです。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_2() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .build();
        let text = "それは何ですか？ペンですか？";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["それは何ですか？", "ペンですか？"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_3() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .build();
        let text = "良かったね！すごい！";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["良かったね！", "すごい！"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_4() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .ex_periods(["\n"])
            .build();
        let text = "良かったね\nすごい！";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["良かったね", "すごい！"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_simple_5() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .ex_periods(["\n", "</br>"])
            .build();
        let text = "良かったね</br>すごい\n";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["良かったね", "すごい"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_1() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .parentheses("「", "」")
            .build();
        let text = "私は「はい。そうです。」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。そうです。」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_2() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .parentheses("（", "）")
            .build();
        let text = "私は「はい。そうです。」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。", "そうです。", "」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_quote_3() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .parentheses("（「", "）」")
            .build();
        let text = "私は「はい。そうです。（嘘だけど。）」と答えた。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["私は「はい。そうです。（嘘だけど。）」と答えた。"];
        assert_eq!(sentences, expected);
    }

    #[test]
    fn test_word_1() {
        let seg = JaSegmenterBuilder::new()
            .in_periods(["。", "？", "！"])
            .parentheses("（「", "）」")
            .no_break_words(["モーニング娘。"])
            .build();
        let text = "モーニング娘。の新曲。";
        let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
        let expected = vec!["モーニング娘。の新曲。"];
        assert_eq!(sentences, expected);
    }
}
