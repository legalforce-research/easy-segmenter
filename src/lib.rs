//! # easy-segmenter
//!
//! Fast and customizable, but easy-to-use, rule-based sentence segmenter.
pub mod basic_ja;
pub mod segmenter;

mod matcher;

pub use segmenter::{Segmenter, SegmenterBuilder};
