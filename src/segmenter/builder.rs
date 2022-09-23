use regex::Regex;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::JaSegmenter;

pub struct JaSegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
    opens: Vec<String>,
    closes: Vec<String>,
    words: Vec<String>,
    regexes: Vec<Regex>,
}

impl JaSegmenterBuilder {
    pub fn new() -> Self {
        Self {
            in_periods: vec![],
            ex_periods: vec![],
            opens: vec![],
            closes: vec![],
            words: vec![],
            regexes: vec![],
        }
    }

    pub fn in_periods<I, P>(mut self, periods: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        periods
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .for_each(|p| self.in_periods.push(p));
        self
    }

    pub fn ex_periods<I, P>(mut self, periods: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        periods
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .for_each(|p| self.ex_periods.push(p));
        self
    }

    pub fn parentheses<P, Q>(mut self, opens: P, closes: Q) -> Self
    where
        P: AsRef<str>,
        Q: AsRef<str>,
    {
        opens
            .as_ref()
            .chars()
            .map(|c| c.to_string())
            .for_each(|c| self.opens.push(c));
        closes
            .as_ref()
            .chars()
            .map(|c| c.to_string())
            .for_each(|c| self.closes.push(c));
        self
    }

    pub fn no_break_words<I, P>(mut self, words: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        words
            .into_iter()
            .map(|w| w.as_ref().to_string())
            .for_each(|w| self.words.push(w));
        self
    }

    pub fn no_break_regex(mut self, regex: Regex) -> Self {
        self.regexes.push(regex);
        self
    }

    pub fn build(self) -> JaSegmenter {
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        let quote_matcher = QuoteMatcher::new(&self.opens, &self.closes);
        let word_matcher = WordMatcher::new(&self.words);
        JaSegmenter::new(period_matcher, quote_matcher, word_matcher, self.regexes)
    }
}
