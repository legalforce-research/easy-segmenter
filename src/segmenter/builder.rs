use crate::matcher::PeriodMatcher;
use crate::segmenter::Segmenter;

#[derive(Default)]
pub struct SegmenterBuilder {
    in_periods: Vec<String>,
    ex_periods: Vec<String>,
}

impl SegmenterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_periods<I, P>(mut self, patterns: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.in_periods = patterns
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .collect();
        self
    }

    pub fn ex_periods<I, P>(mut self, patterns: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        self.ex_periods = patterns
            .into_iter()
            .map(|p| p.as_ref().to_string())
            .collect();
        self
    }

    pub fn build(self) -> Segmenter {
        let period_matcher = PeriodMatcher::new(&self.in_periods, &self.ex_periods);
        Segmenter::new(period_matcher)
    }
}
