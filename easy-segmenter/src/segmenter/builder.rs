use regex::Regex;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::Segmenter;

/// Builder of [`Segmenter`].
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

    /// Defines periods for breaking segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new().in_periods(["。", "？"]).build();
    /// let text = "それは何ですか？ペンです。";
    /// let segments: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["それは何ですか？", "ペンです。"];
    /// assert_eq!(segments, expected);
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

    /// Defines periods for breaking segments.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new().ex_periods(["\n"]).build();
    /// let text = "これはペンです\nそれはマーカーです\n";
    /// let segments: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["これはペンです", "それはマーカーです"];
    /// assert_eq!(segments, expected);
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

    /// Defines periods for breaking segments.
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
    /// let segments: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["私は「はい。そうです。」と答えた。"];
    /// assert_eq!(segments, expected);
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

    /// Defines periods for breaking segments.
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
    /// let segments: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["モーニング娘。の新曲"];
    /// assert_eq!(segments, expected);
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

    /// Defines periods for breaking segments.
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
    /// let text = "３．１４１５９２";
    /// let segments: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["３．１４１５９２"];
    /// assert_eq!(segments, expected);
    /// ```
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
