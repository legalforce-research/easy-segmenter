//! Builder of [`Segmenter`] to define segmentation rules.
use regex::Regex;

use crate::errors::{EasySegmenterError, Result};
use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::Segmenter;

/// The default value of the maximum level of nested parentheses handled as quotations.
pub const DEFAULT_MAX_QUOTE_LEVEL: usize = 3;

/// Builder of [`Segmenter`] to define segmentation rules.
///
/// This class allows rules to be defined from scratch.
/// You can also use template rules in [`crate::template`].
pub struct SegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
    parentheses: Vec<(char, char)>,
    words: Vec<String>,
    regexes: Vec<Regex>,
    max_quote_level: usize,
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
            max_quote_level: DEFAULT_MAX_QUOTE_LEVEL,
        }
    }

    /// Compiles the segmenter.
    pub fn build(self) -> Result<Segmenter> {
        if self.in_periods.is_empty() && self.ex_periods.is_empty() {
            return Err(EasySegmenterError::input(
                "Both in_ and ex_periods must not be empty.",
            ));
        }
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        let quote_matcher = if self.parentheses.is_empty() {
            None
        } else {
            Some(QuoteMatcher::new(&self.parentheses)?)
        };
        let word_matcher = if self.words.is_empty() {
            None
        } else {
            Some(WordMatcher::new(&self.words))
        };
        Ok(Segmenter::new(
            period_matcher,
            quote_matcher,
            word_matcher,
            self.regexes,
            self.max_quote_level,
        ))
    }

    /// Adds periods that break texts and are included in resulting sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_periods(["。", "？"])
    ///     .build()
    ///     .unwrap();
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
    /// let seg = SegmenterBuilder::new()
    ///     .ex_periods(["\n"])
    ///     .build()
    ///     .unwrap();
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
    /// `parentheses` must not have duplicate entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_periods(["。"])
    ///     .parentheses([('「', '」')])
    ///     .build()
    ///     .unwrap();
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
    ///     .build()
    ///     .unwrap();
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
    ///     .build()
    ///     .unwrap();
    /// let text = "３．１４";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["３．１４"];
    /// assert_eq!(sentences, expected);
    /// ```
    ///
    /// # Notes
    ///
    /// This function takes a single regex pattern, not a sequence of those, because
    ///  - many regex patterns should be registered as a general rule, and
    ///  - a single regex can define multiple rules.
    /// Nonetheless, you can register multiple patterns by repeating this function.
    pub fn no_break_regex(mut self, regex: Regex) -> Self {
        self.regexes.push(regex);
        self
    }

    /// Sets the maximum level of nested parentheses handled as quotations.
    /// The default value is [`DEFAULT_MAX_QUOTE_LEVEL`].
    ///
    /// A smaller value will speed up segmentation but
    /// make it more susceptible to errant parenthesis pairs.
    ///
    /// # Errors
    ///
    /// An error will arise when `max_quote_level == 0`.
    pub fn max_quote_level(mut self, max_quote_level: usize) -> Result<Self> {
        if max_quote_level == 0 {
            Err(EasySegmenterError::input(
                "max_quote_level must not be zero.",
            ))
        } else {
            self.max_quote_level = max_quote_level;
            Ok(self)
        }
    }
}
