use std::ops::Range;

pub struct Bitset {
    bits: Vec<u64>,
}

impl Bitset {
    pub fn new(len: usize) -> Self {
        Self {
            bits: vec![0; Self::words_for(len)],
        }
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> bool {
        let (block, shift) = (i / 64, i % 64);
        (self.bits[block] >> shift) & 1 == 1
    }

    #[inline(always)]
    pub fn set_range(&mut self, r: Range<usize>) {
        if r.end == 0 {
            return;
        }

        let start_shift = r.start % 64;
        let end_shift = r.end % 64;
        let start_block = r.start / 64;
        let last_block = if end_shift == 0 {
            r.end / 64 - 1
        } else {
            r.end / 64
        };

        for block in start_block..=last_block {
            let mut mask = u64::MAX;
            if block == start_block {
                // Clear the first start_shift bits.
                mask >>= start_shift;
                mask <<= start_shift;
            }
            if block == last_block && end_shift != 0 {
                // Clear the bits other than the first end_shift bits.
                mask <<= 64 - end_shift;
                mask >>= 64 - end_shift;
            }
            self.bits[block] |= mask;
        }
    }

    #[inline(always)]
    const fn words_for(n: usize) -> usize {
        (n + 63) / 64
    }
}
