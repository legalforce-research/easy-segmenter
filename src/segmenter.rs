//! Fast and customizable, but easy-to-use, rule-based sentence segmenter.
pub mod builder;
pub use builder::SegmenterBuilder;

#[cfg(test)]
mod tests;

use regex::Regex;

use crate::bitset::Bitset;
use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::template;

/// Fast and customizable, but easy-to-use, rule-based sentence segmenter.
///
/// # Examples
///
/// ```rust
/// use easy_segmenter::Segmenter;
///
/// let seg = Segmenter::with_template_ja_config();
/// let text = "円周率はいくつですか？３．１４です。なるほど、\
///     以前に「３の方が良いのでは？」と聞いた気がしますが\n今も３．１４なんですね";
/// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
/// let expected = vec![
///     "円周率はいくつですか？",
///     "３．１４です。",
///     "なるほど、以前に「３の方が良いのでは？」と聞いた気がしますが",
///     "今も３．１４なんですね",
/// ];
/// assert_eq!(sentences, expected);
/// ```
pub struct Segmenter {
    // Breakers
    period_matcher: PeriodMatcher,
    // Non Breakers
    quote_matcher: Option<QuoteMatcher>,
    word_matcher: Option<WordMatcher>,
    regex_matchers: Vec<Regex>,
    max_quote_level: usize,
}

impl Segmenter {
    fn new(
        period_matcher: PeriodMatcher,
        quote_matcher: Option<QuoteMatcher>,
        word_matcher: Option<WordMatcher>,
        regex_matchers: Vec<Regex>,
        max_quote_level: usize,
    ) -> Self {
        Self {
            period_matcher,
            quote_matcher,
            word_matcher,
            regex_matchers,
            max_quote_level,
        }
    }

    /// Creates an instance with basic segmentation rules defined in [`template::ja`].
    pub fn with_template_ja_config() -> Self {
        SegmenterBuilder::new()
            .in_periods(template::ja::in_periods())
            .ex_periods(template::ja::ex_periods())
            .parentheses(template::ja::parentheses())
            .no_break_regex(template::ja::decimal_point())
            .build()
            .unwrap()
    }

    /// Segments an input text into sentences, returning byte-position ranges.
    pub fn segment<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut no_break = Bitset::new(text.len());

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
            if end_pos != 0 && no_break.get(end_pos - 1) {
                None
            } else if start_pos == end_pos {
                start_pos = m.end;
                None
            } else {
                let range = (start_pos, end_pos);
                start_pos = m.end;
                Some(range)
            }
        })
    }

    fn find_quotes(&self, text: &str, detected: &mut Bitset) {
        if let Some(quote_matcher) = self.quote_matcher.as_ref() {
            let mut stack = vec![];
            for m in quote_matcher.iter(text) {
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
                    detected.set_range(start..m.end);
                }
                stack.pop();
            }
        }
    }

    fn find_words(&self, text: &str, detected: &mut Bitset) {
        if let Some(word_matcher) = self.word_matcher.as_ref() {
            for m in word_matcher.iter(text) {
                detected.set_range(m.start..m.end);
            }
        }
    }

    fn find_regex(&self, text: &str, detected: &mut Bitset) {
        for re in &self.regex_matchers {
            for cap in re.captures_iter(text) {
                for idx in 1..cap.len() {
                    if let Some(m) = cap.get(idx) {
                        detected.set_range(m.range());
                    }
                }
            }
        }
    }
}
