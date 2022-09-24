//! # easy-segmenter
//!
//! Fast and customizable, but easy-to-use, rule-based sentence segmenter.
pub mod errors;
pub mod segmenter;
pub mod template;

mod matcher;

pub use segmenter::{Segmenter, SegmenterBuilder};
