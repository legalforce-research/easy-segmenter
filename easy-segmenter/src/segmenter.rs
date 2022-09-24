pub mod builder;
pub use builder::SegmenterBuilder;

#[cfg(test)]
mod tests;

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
