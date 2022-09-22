pub mod builder;

use crate::matcher::PeriodMatcher;

pub struct Segmenter {
    period_matcher: PeriodMatcher,
}

impl Segmenter {
    fn new(period_matcher: PeriodMatcher) -> Self {
        Self { period_matcher }
    }
}
