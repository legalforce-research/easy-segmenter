use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use hashbrown::HashMap;

use crate::errors::{EasySegmenterError, Result};

pub struct PeriodMatch {
    pub start: usize,
    pub end: usize,
    pub is_in_period: bool,
}

pub struct PeriodMatcher {
    pma: AhoCorasick,
    num_in_periods: usize,
}

impl PeriodMatcher {
    pub fn new<P>(in_periods: &[P], ex_periods: &[P]) -> Self
    where
        P: AsRef<str>,
    {
        let mut patterns = vec![];
        in_periods
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        ex_periods
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        let pma = AhoCorasickBuilder::new()
            .auto_configure(&patterns)
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns);
        Self {
            pma,
            num_in_periods: in_periods.len(),
        }
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = PeriodMatch> + 'a {
        self.pma
            .find_iter(text)
            .map(move |m| PeriodMatch {
                start: m.start(),
                end: m.end(),
                is_in_period: m.pattern() < self.num_in_periods,
            })
            // Always returns an imaginary terminator to address the case that
            // the last character does not have any period.
            .chain([PeriodMatch {
                start: text.len(),
                end: text.len(),
                is_in_period: false,
            }])
    }
}

pub struct QuoteMatch {
    pub start: usize,
    pub end: usize,
    pub id: usize,
    pub is_open: bool,
}

pub struct QuoteMatcher {
    map: HashMap<char, usize>,
}

impl QuoteMatcher {
    pub fn new(parentheses: &[(char, char)]) -> Result<Self> {
        let mut map = HashMap::new();
        for (i, &(p, q)) in parentheses.iter().enumerate() {
            if map.insert(p, i * 2).is_some() {
                return Err(EasySegmenterError::input(format!(
                    "{p} has been already registered. Entries must be unique."
                )));
            }
            if map.insert(q, i * 2 + 1).is_some() {
                return Err(EasySegmenterError::input(format!(
                    "{q} has been already registered. Entries must be unique."
                )));
            }
        }
        Ok(Self { map })
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = QuoteMatch> + 'a {
        let mut end = 0;
        text.chars().filter_map(move |c| {
            let len = c.len_utf8();
            let start = end;
            end += len;
            self.map.get(&c).cloned().map(|v| {
                let id = v / 2;
                let is_open = v % 2 == 0;
                QuoteMatch {
                    start,
                    end,
                    id,
                    is_open,
                }
            })
        })
    }
}

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
