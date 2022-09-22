use aho_corasick::AhoCorasick;

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
}
