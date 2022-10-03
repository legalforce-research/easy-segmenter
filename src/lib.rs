//! # easy-segmenter
//!
//! easy-segmenter is a fast and customizable rule-based sentence segmenter library for Rust.
//!
//! ## Getting started
//!
//! ```rust
//! let seg = easy_segmenter::Segmenter::with_template_ja_config();
//!
//! let text = "円周率はいくつですか？３．１４です。なるほど\n以前に「３の方が良いのでは？」と聞いた気がします";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec![
//!     "円周率はいくつですか？",
//!     "３．１４です。",
//!     "なるほど",
//!     "以前に「３の方が良いのでは？」と聞いた気がします",
//! ];
//! assert_eq!(sentences, expected);
//! ```
//!
//! `with_template_ja_config()` creates a segmenter with basic segmentation rules that
//! are enhanced from the [Golden Rules](https://github.com/diasks2/pragmatic_segmenter#golden-rules-japanese) in [Pragmatic Segmenter](https://github.com/diasks2/pragmatic_segmenter).
//! See the contents of [`template`] for the definition.
//!
//! Also, you can manually define segmentation rules as follows.
//!
//! ## How to customize
//!
//! `easy_segmenter::Segmenter` does not hardcode any segmentation rules and
//! provides a simple framework of rule definitions.
//!
//! You only need to define the two types of rules:
//! - what to segment sentences by, and
//! - what not to segment.
//!
//! ### Delimiters for segmenting sentences
//!
//! easy-segmenter handles two types of sentence delimiters:
//!
//! - *Inclusive delimiters* that are included in resulting sentences, and
//! - *Exclusive delimiters* that are excluded in resulting sentences.
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["。"]) // Inclusive delimiters
//!     .ex_delimiters(["\n"]) // Exclusive delimiters
//!     .build()
//!     .unwrap();
//! let text = "なるほど\nその通りですね。";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["なるほど", "その通りですね。"]; // "\n" is excluded.
//! assert_eq!(sentences, expected);
//! ```
//!
//! Sentence delimiters are detected with exact string matching for a set of patterns.
//! If multiple delimiters are overlapped at a position,
//! the [leftmost-longest one](https://docs.rs/aho-corasick/latest/aho_corasick/enum.MatchKind.html#variant.LeftmostLongest) is detected.
//! The match semantics allow for handling specific cases such as carriage returns and multiple dots.
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["。", "。。。"])
//!     .ex_delimiters(["\n", "\r\n", "\r"])
//!     .build()
//!     .unwrap();
//! let text = "なるほど。。。その通りですね\r\n";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["なるほど。。。", "その通りですね"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! Itemization can be also handled.
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .ex_delimiters(["\n", "\n・"])
//!     .build()
//!     .unwrap();
//! let text = "買うもの\n・ご飯\n・卵\n・醤油\n計３点";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["買うもの", "ご飯", "卵", "醤油", "計３点"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! ### Rules for not segmenting sentences
//!
//! easy-segmenter provides three ways to define rules for not segmenting sentences.
//! These rules always take priority over sentence delimiters.
//!
//! #### 1. Quotation
//!
//! Quoted sentences will not be segmented.
//! You can define pairs of characters to specify quotations such as brackets and
//! parentheses.
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["。"])
//!     .quotes([('「', '」')])
//!     .build()
//!     .unwrap();
//! let text = "私は「はい。そうです。」と答えた。";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["私は「はい。そうです。」と答えた。"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! #### 2. Words
//!
//! You can define words that should not be segmented.
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["。"])
//!     .no_break_words(["モーニング娘。"])
//!     .build()
//!     .unwrap();
//! let text = "モーニング娘。の新曲";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["モーニング娘。の新曲"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! #### 3. Regex
//!
//! You can define regex patterns that should not be segmented.
//! Captured patterns will not be segmented.
//!
//! - Example 1. Handling decimal points
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["．"])
//!     .no_break_regex(regex::Regex::new(r"\d(．)\d").unwrap())
//!     .build()
//!     .unwrap();
//! let text = "３．１４";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["３．１４"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! - Example 2. Handling dot sequences
//!
//! ```rust
//! let seg = easy_segmenter::SegmenterBuilder::new()
//!     .in_delimiters(["。"])
//!     .no_break_regex(regex::Regex::new(r"(。{2,})。").unwrap())
//!     .build()
//!     .unwrap();
//! let text = "はぁ。。。。。疲れた。。。";
//! let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
//! let expected = vec!["はぁ。。。。。", "疲れた。。。"];
//! assert_eq!(sentences, expected);
//! ```
//!
//! Regular expressions are powerful, but complicated ones can slow down segmentation.
//! *Consider using `no_break_words` first to solve your problem.*
//!
//! ## Not supported by easy-segmenter
//!
//! For simplicity, easy-segmenter does not support any function that requires editing of the original text,
//! although such functions are often supported by other tools.
//!
//! ### Normalization
//!
//! The method of normalization depends on your application, and there are several possible methods
//! such as [neologd](https://github.com/neologd/mecab-ipadic-neologd/wiki/Regexp).
//! Therefore, it should not be included in easy-segmenter.
//!
//! ### Fixing errant line breaks
//!
//! Some other tools erase line breaks that are erroneously inserted in a sentence.
//!
//! ```text
//! "新しい\n教科書" => ["新しい教科書"]
//! ```
//!
//! easy-segmenter does not fix such errors because whether or not it is an error depends on the data.
//! Those errors should be corrected in pre- or post-processing using NLP techniques such as [Hayashibe and Mitsuzawa, W-NUT 2020](https://aclanthology.org/2020.wnut-1.10.pdf).
//!
//! ### Quotation blocks
//!
//! Quotation blocks like below are not also corrected in easy-segmenter with the same reason.
//!
//! ```text
//! >> コーディングが好きなソフトウェ
//! >> アエンジニアや研究が好きなリサ
//! >> ーチエンジニアを募集しています
//! ```
//!
//! However, easy-segmenter will be useful to remove those quotation markers in preprocessing.
//! It can be achived by segmenting the original text with `ex_delimiters(["\n>> "])` and concatenating the resulting sentences.
#![deny(missing_docs)]

pub mod errors;
pub mod segmenter;
pub mod template;

mod bitset;
mod matcher;

pub use segmenter::{Segmenter, SegmenterBuilder};
