use regex::Regex;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::Segmenter;

pub struct SegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
    parentheses: Vec<(char, char)>,
    words: Vec<String>,
    regexes: Vec<Regex>,
}

impl SegmenterBuilder {
    pub const fn new() -> Self {
        Self {
            in_periods: vec![],
            ex_periods: vec![],
            parentheses: vec![],
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

    pub fn parentheses<I>(mut self, parentheses: I) -> Self
    where
        I: IntoIterator<Item = (char, char)>,
    {
        parentheses
            .into_iter()
            .for_each(|p| self.parentheses.push(p));
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

    pub fn build(self) -> Segmenter {
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        let quote_matcher = QuoteMatcher::new(&self.parentheses);
        let word_matcher = WordMatcher::new(&self.words);
        Segmenter::new(period_matcher, quote_matcher, word_matcher, self.regexes)
    }
}
