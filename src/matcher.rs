use aho_corasick::AhoCorasick;

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
        let pma = AhoCorasick::new_auto_configured(&patterns);
        Self {
            pma,
            num_in_periods: in_periods.len(),
        }
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = PeriodMatch> + 'a {
        self.pma
            .find_overlapping_iter(text)
            .map(move |m| PeriodMatch {
                start: m.start(),
                end: m.end(),
                is_in_period: m.pattern() < self.num_in_periods,
            })
    }
}

pub struct QuoteMatch {
    pub start: usize,
    pub end: usize,
    pub is_open: bool,
}

pub struct QuoteMatcher {
    pma: AhoCorasick,
    num_opens: usize,
}

impl QuoteMatcher {
    pub fn new<P>(opens: &[P], closes: &[P]) -> Self
    where
        P: AsRef<str>,
    {
        let mut patterns = vec![];
        opens
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        closes
            .iter()
            .map(|p| p.as_ref())
            .for_each(|p| patterns.push(p));
        let pma = AhoCorasick::new_auto_configured(&patterns);
        Self {
            pma,
            num_opens: opens.len(),
        }
    }

    pub fn iter<'a>(&'a self, text: &'a str) -> impl Iterator<Item = QuoteMatch> + 'a {
        self.pma
            .find_overlapping_iter(text)
            .map(move |m| QuoteMatch {
                start: m.start(),
                end: m.end(),
                is_open: m.pattern() < self.num_opens,
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
