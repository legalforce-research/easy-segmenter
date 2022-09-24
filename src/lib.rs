//! # easy-segmenter
//!
//! Fast and customizable, but easy-to-use, rule-based sentence segmenter.
pub mod basic;
pub mod segmenter;

mod matcher;

pub use segmenter::{Segmenter, SegmenterBuilder};
