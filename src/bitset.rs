use std::ops::Range;

pub struct Bitset {
    bits: Vec<bool>,
}

impl Bitset {
    pub fn new(len: usize) -> Self {
        Self {
            bits: vec![false; len],
        }
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> bool {
        self.bits[i]
    }

    #[inline(always)]
    pub fn set_range(&mut self, r: Range<usize>) {
        for i in r {
            self.bits[i] = true
        }
    }
}
