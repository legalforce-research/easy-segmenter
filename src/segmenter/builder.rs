//! Builder of [`Segmenter`] to define segmentation rules.
use regex::Regex;

use crate::errors::{EasySegmenterError, Result};
use crate::matcher::{DelimiterMatcher, QuoteMatcher, WordMatcher};
use crate::segmenter::Segmenter;

/// The default value of the maximum nesting level for quotations.
pub const DEFAULT_MAX_QUOTE_LEVEL: usize = 3;

/// Builder of [`Segmenter`] to define segmentation rules.
/// This class allows rules to be defined from scratch.
/// You can also use template rules in [`crate::template`].
///
/// # Rules in delimiters
///
/// Delimiters are detected with exact string matching for a set of patterns.
/// If multiple delimiters are overlapped at a position,
/// the [leftmost-longest one](https://docs.rs/aho-corasick/latest/aho_corasick/enum.MatchKind.html#variant.LeftmostLongest) is detected.
pub struct SegmenterBuilder {
    in_delimiters: Vec<String>,
    ex_delimiters: Vec<String>,
    quotes: Vec<(char, char)>,
    words: Vec<String>,
    regexes: Vec<Regex>,
    max_quote_level: usize,
}

impl SegmenterBuilder {
    /// Creates an instance.
    pub const fn new() -> Self {
        Self {
            in_delimiters: vec![],
            ex_delimiters: vec![],
            quotes: vec![],
            words: vec![],
            regexes: vec![],
            max_quote_level: DEFAULT_MAX_QUOTE_LEVEL,
        }
    }

    /// Compiles the segmenter.
    pub fn build(self) -> Result<Segmenter> {
        if self.in_delimiters.is_empty() && self.ex_delimiters.is_empty() {
            return Err(EasySegmenterError::input(
                "Both in_ and ex_delimiters must not be empty.",
            ));
        }
        let delimiter_matcher = DelimiterMatcher::new(&self.in_delimiters, &self.ex_delimiters);
        let quote_matcher = if self.quotes.is_empty() {
            None
        } else {
            Some(QuoteMatcher::new(&self.quotes)?)
        };
        let word_matcher = if self.words.is_empty() {
            None
        } else {
            Some(WordMatcher::new(&self.words))
        };
        Ok(Segmenter::new(
            delimiter_matcher,
            quote_matcher,
            word_matcher,
            self.regexes,
            self.max_quote_level,
        ))
    }

    /// Adds delimiters that break texts and are included in resulting sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_delimiters(["。", "？"])
    ///     .build()
    ///     .unwrap();
    /// let text = "それは何ですか？ペンです。";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["それは何ですか？", "ペンです。"];
    /// assert_eq!(sentences, expected);
    /// ```
    pub fn in_delimiters<I, P>(mut self, delimiters: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        delimiters
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .for_each(|p| self.in_delimiters.push(p));
        self
    }

    /// Adds delimiters that break texts and are excluded in resulting sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .ex_delimiters(["\n"])
    ///     .build()
    ///     .unwrap();
    /// let text = "これはペンです\nそれはマーカーです\n";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["これはペンです", "それはマーカーです"];
    /// assert_eq!(sentences, expected);
    /// ```
    pub fn ex_delimiters<I, P>(mut self, delimiters: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        delimiters
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .for_each(|p| self.ex_delimiters.push(p));
        self
    }

    /// Adds character pairs to specify the start and end of a quotation.
    /// Sentences within a quotation will not be broken.
    ///
    /// `quotes` must not have duplicate entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use easy_segmenter::SegmenterBuilder;
    ///
    /// let seg = SegmenterBuilder::new()
    ///     .in_delimiters(["。"])
    ///     .quotes([('「', '」')])
    ///     .build()
    ///     .unwrap();
    /// let text = "私は「はい。そうです。」と答えた。";
    /// let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    /// let expected = vec!["私は「はい。そうです。」と答えた。"];
    /// assert_eq!(sentences, expected);
    /// ```
    pub fn quotes<I>(mut self, quotes: I) -> Self
    where
        I: IntoIterator<Item = (char, char)>,
    {
        quotes.into_iter().for_each(|p| self.quotes.push(p));
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
    ///     .in_delimiters(["。"])
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
    ///     .in_delimiters(["．"])
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

    /// Sets the maximum nesting level for quotations.
    /// The default value is [`DEFAULT_MAX_QUOTE_LEVEL`].
    ///
    /// A smaller value will speed up segmentation but
    /// make it more susceptible to errant pairs of quotation marks.
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
