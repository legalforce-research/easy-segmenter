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

    #[allow(clippy::missing_const_for_fn)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_range_1() {
        let mut bitset = Bitset::new(200);
        bitset.set_range(0..200);
        for i in 0..200 {
            assert!(bitset.get(i));
        }
    }

    #[test]
    fn test_set_range_2() {
        let mut bitset = Bitset::new(200);
        bitset.set_range(10..190);
        for i in 0..10 {
            assert!(!bitset.get(i));
        }
        for i in 10..190 {
            assert!(bitset.get(i));
        }
        for i in 190..200 {
            assert!(!bitset.get(i));
        }
    }

    #[test]
    fn test_set_range_3() {
        let mut bitset = Bitset::new(200);
        bitset.set_range(64..128);
        for i in 0..64 {
            assert!(!bitset.get(i));
        }
        for i in 64..128 {
            assert!(bitset.get(i));
        }
        for i in 128..200 {
            assert!(!bitset.get(i));
        }
    }

    #[test]
    fn test_set_range_4() {
        let mut bitset = Bitset::new(200);
        bitset.set_range(65..127);
        for i in 0..65 {
            assert!(!bitset.get(i));
        }
        for i in 65..127 {
            assert!(bitset.get(i));
        }
        for i in 127..200 {
            assert!(!bitset.get(i));
        }
    }
}
