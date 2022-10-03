use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

use crate::errors::{EasySegmenterError, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct DelimiterMatch {
    pub start: usize,
    pub end: usize,
    pub is_in_delimiter: bool,
}

pub struct DelimiterMatcher {
    pma: AhoCorasick,
    num_in_delimiters: usize,
}

impl DelimiterMatcher {
    pub fn new<P>(in_delimiters: &[P], ex_delimiters: &[P]) -> Self
    where
        P: AsRef<str>,
    {
        let mut patterns = vec![];
        in_delimiters
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        ex_delimiters
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        let pma = AhoCorasickBuilder::new()
            .auto_configure(&patterns)
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns);
        Self {
            pma,
            num_in_delimiters: in_delimiters.len(),
        }
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = DelimiterMatch> + 'a {
        self.pma
            .find_iter(text)
            .map(move |m| DelimiterMatch {
                start: m.start(),
                end: m.end(),
                is_in_delimiter: m.pattern() < self.num_in_delimiters,
            })
            // Always returns an imaginary terminator to address the case that
            // the last character does not have any delimiter.
            .chain([DelimiterMatch {
                start: text.len(),
                end: text.len(),
                is_in_delimiter: false,
            }])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct QuoteMatch {
    pub start: usize,
    pub end: usize,
    pub id: usize,
    pub is_open: bool,
}

pub struct QuoteMatcher {
    pma: AhoCorasick,
}

impl QuoteMatcher {
    pub fn new(quotes: &[(char, char)]) -> Result<Self> {
        let mut patterns = vec![];
        for &(p, q) in quotes {
            patterns.push(p.to_string());
            patterns.push(q.to_string());
        }
        if !is_unique(&patterns) {
            return Err(EasySegmenterError::input("Entries must be unique."));
        }
        let pma = AhoCorasickBuilder::new()
            .auto_configure(&patterns)
            .build(&patterns);
        Ok(Self { pma })
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = QuoteMatch> + 'a {
        self.pma.find_iter(text).map(move |m| {
            let id = m.pattern() / 2;
            let is_open = m.pattern() % 2 == 0;
            QuoteMatch {
                start: m.start(),
                end: m.end(),
                id,
                is_open,
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordMatch {
    pub start: usize,
    pub end: usize,
}

pub struct WordMatcher {
    pma: AhoCorasick,
}

impl WordMatcher {
    pub fn new<P>(words: &[P]) -> Self
    where
        P: AsRef<str>,
    {
        let patterns: Vec<_> = words.iter().map(|p| p.as_ref()).collect();
        let pma = AhoCorasick::new_auto_configured(&patterns);
        Self { pma }
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = WordMatch> + 'a {
        self.pma
            .find_overlapping_iter(text)
            .map(move |m| WordMatch {
                start: m.start(),
                end: m.end(),
            })
    }
}

fn is_unique<S>(x: &[S]) -> bool
where
    S: AsRef<str>,
{
    let mut v: Vec<_> = x.iter().map(|s| s.as_ref()).collect();
    v.sort_unstable();
    v.dedup();
    v.len() == x.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_1() {
        let quotes = vec![('「', '」'), ('（', '）')];
        let matcher = QuoteMatcher::new(&quotes).unwrap();
        let matches: Vec<_> = matcher.iter("「」（）").collect();
        let expected = vec![
            QuoteMatch {
                start: 0,
                end: 3,
                id: 0,
                is_open: true,
            },
            QuoteMatch {
                start: 3,
                end: 6,
                id: 0,
                is_open: false,
            },
            QuoteMatch {
                start: 6,
                end: 9,
                id: 1,
                is_open: true,
            },
            QuoteMatch {
                start: 9,
                end: 12,
                id: 1,
                is_open: false,
            },
        ];
        assert_eq!(matches, expected);
    }

    #[test]
    fn test_quote_2() {
        let quotes = vec![('「', '」'), ('（', '」')];
        assert!(QuoteMatcher::new(&quotes).is_err());
    }
}
