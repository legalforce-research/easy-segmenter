# easy-segmenter

easy-segmenter is a fast and customizable rule-based sentence segmenter library for Rust.

## Features

- Easy-to-use: easy-segmenter provides pre-defined segmentation rules for supported
  languages such as Japanese.
- Customizable: easy-segmenter provides flexible APIs to define new custom segmentation
  rules.
- Extensible: easy-segmenter is originally designed for Japanese, but it is possible to
  add support for other languages through APIs.
- Self-contained: sentence segmentation is performed solely by segmentation rules
  without relying on external resources.

## Getting started

```rust
let seg = easy_segmenter::Segmenter::with_template_ja_config();

let text = "円周率はいくつですか？３．１４です。なるほど\n以前に「３の方が良いのでは？」と聞いた気がします";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec![
    "円周率はいくつですか？",
    "３．１４です。",
    "なるほど",
    "以前に「３の方が良いのでは？」と聞いた気がします",
];
assert_eq!(sentences, expected);
```

`with_template_ja_config()` creates a segmenter with basic segmentation rules that
are enhanced from the [Golden Rules](https://github.com/diasks2/pragmatic_segmenter#golden-rules-japanese) in [Pragmatic Segmenter](https://github.com/diasks2/pragmatic_segmenter).

## API documentation

See the API documentation for detailed usage.
(Please run `cargo doc` since it has not been published in crates.io yet.)

## Disclaimer

This software is developed by LegalForce, Inc.,
but not an officially supported LegalForce product.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
