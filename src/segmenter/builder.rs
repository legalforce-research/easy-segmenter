//! Builder of [`Segmenter`] to define segmentation rules.
use regex::Regex;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::Segmenter;

/// Builder of [`Segmenter`] to define segmentation rules.
pub struct SegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
    parentheses: Vec<(char, char)>,
    words: Vec<String>,
    regexes: Vec<Regex>,
}

impl SegmenterBuilder {
    /// Creates an instance.
    pub const fn new() -> Self {
        Self {
            in_periods: vec![],
            ex_periods: vec![],
            parentheses: vec![],
            words: vec![],
            regexes: vec![],
        }
    }

    /// Compiles the segmenter.
    pub fn build(self) -> Segmenter {
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        let quote_matcher = QuoteMatcher::new(&self.parentheses);
        let word_matcher = WordMatcher::new(&self.words);
        Segmenter::new(period_matcher, quote_matcher, word_matcher, self.regexes)
    }

    /// Adds periods that break texts and are included in resulting sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new().in_periods(["。", "？"]).build();
    /// let text = "それは何ですか？ペンです。";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["それは何ですか？", "ペンです。"];
    /// assert_eq!(sentences, expected);
    /// ```
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

    /// Adds periods that break texts and are excluded in resulting sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new().ex_periods(["\n"]).build();
    /// let text = "これはペンです\nそれはマーカーです\n";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["これはペンです", "それはマーカーです"];
    /// assert_eq!(sentences, expected);
    /// ```
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

    /// Adds parentheses to specify quotations.
    /// Sentences within a quotation will not be broken.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_periods(["。"])
    ///     .parentheses([('「', '」')])
    ///     .build();
    /// let text = "私は「はい。そうです。」と答えた。";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["私は「はい。そうです。」と答えた。"];
    /// assert_eq!(sentences, expected);
    /// ```
    pub fn parentheses<I>(mut self, parentheses: I) -> Self
    where
        I: IntoIterator<Item = (char, char)>,
    {
        parentheses
            .into_iter()
            .for_each(|p| self.parentheses.push(p));
        self
    }

    /// Adds words that should not be broken.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_periods(["。"])
    ///     .no_break_words(["モーニング娘。"])
    ///     .build();
    /// let text = "モーニング娘。の新曲";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["モーニング娘。の新曲"];
    /// assert_eq!(sentences, expected);
    /// ```
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

    /// Adds regex patterns that should not be broken.
    /// Captured patterns will not be broken.
    ///
    /// Regular expressions are powerful, but complicated ones can slow down segmentation.
    /// Consider using [`Self::no_break_words`] first to solve your problem.
    ///
    /// # Examples
    ///
    /// ```
    /// use regex::Regex;
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_periods(["．"])
    ///     .no_break_regex(Regex::new(r"\d(．)\d").unwrap())
    ///     .build();
    /// let text = "３．１４";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["３．１４"];
    /// assert_eq!(sentences, expected);
    /// ```
    pub fn no_break_regex(mut self, regex: Regex) -> Self {
        self.regexes.push(regex);
        self
    }
}
