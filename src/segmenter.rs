pub mod builder;

use crate::matcher::{PeriodMatcher, QuoteMatcher, WordMatcher};

pub struct JaSegmenter {
    period_matcher: PeriodMatcher,
    quote_matcher: QuoteMatcher,
    word_matcher: WordMatcher,
}

impl JaSegmenter {
    fn new(
        period_matcher: PeriodMatcher,
        quote_matcher: QuoteMatcher,
        word_matcher: WordMatcher,
    ) -> Self {
        Self {
            period_matcher,
            quote_matcher,
            word_matcher,
        }
    }

    pub fn segment<'a>(&'a self, text: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut no_split = vec![false; text.len()];
        self.find_quotes(text, &mut no_split);
        self.find_words(text, &mut no_split);

        let mut start_pos = 0;
        self.period_matcher.iter(text).filter_map(move |m| {
            // if is_in_period, the period should be inclusive in the segment;
            // otherwise, the period should be exclusive in the segment.
            let end_pos = if m.is_in_period { m.end } else { m.start };
            if end_pos != 0 && no_split[end_pos - 1] {
                None
            } else {
                let range = (start_pos, end_pos);
                start_pos = m.end;
                Some(range)
            }
        })
    }

    fn find_quotes(&self, text: &str, detected: &mut [bool]) {
        let mut level = 0;
        let mut open_start = 0;
        for m in self.quote_matcher.iter(text) {
            if m.is_open {
                level += 1;
                open_start = m.start;
            } else if level > 0 {
                level -= 1;
                if level == 0 {
                    for i in open_start..m.end {
                        detected[i] = true;
                    }
                }
            }
        }
    }

    fn find_words(&self, text: &str, detected: &mut [bool]) {
        for m in self.word_matcher.iter(text) {
            for i in m.start..m.end {
                detected[i] = true;
            }
        }
    }
}
