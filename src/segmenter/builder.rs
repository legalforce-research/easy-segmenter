use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::JaSegmenter;

#[derive(Default)]
pub struct JaSegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
    opens: Vec<String>,
    closes: Vec<String>,
    words: Vec<String>,
}

impl JaSegmenterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_periods<I, P>(mut self, periods: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.in_periods = periods
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .collect();
        self
    }

    pub fn ex_periods<I, P>(mut self, periods: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.ex_periods = periods
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .collect();
        self
    }

    pub fn parentheses<P, Q>(mut self, opens: P, closes: Q) -> Self
    where
        P: AsRef<str>,
        Q: AsRef<str>,
    {
        self.opens = opens.as_ref().chars().map(|c| c.to_string()).collect();
        self.closes = closes.as_ref().chars().map(|c| c.to_string()).collect();
        self
    }

    pub fn words<I, P>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.words = words.into_iter().map(|p| p.as_ref().to_string()).collect();
        self
    }

    pub fn build(self) -> JaSegmenter {
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        let quote_matcher = QuoteMatcher::new(&self.opens, &self.closes);
        let word_matcher = WordMatcher::new(&self.words);
        JaSegmenter::new(period_matcher, quote_matcher, word_matcher)
    }
}
