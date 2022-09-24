# easy-segmenter

Fast and customizable, but easy-to-use, rule-based sentence segmenter.

## Basic usage

## Customizability

`easy_segmenter::Segmenter` does not hardcode any segmentation rules, and
provides a simple framework of rule definitions.

### Periods

```rust
use easy_segmenter::SegmenterBuilder;

let seg = SegmenterBuilder::new()
    .in_periods(["。"])
    .ex_periods(["\n"])
    .build();
let text = "なるほど\nその通りですね。";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["なるほど", "その通りですね。"];
assert_eq!(sentences, expected);
```

### Rules for not segmenting sentences

#### Quotation

```rust
use easy_segmenter::SegmenterBuilder;

let seg = SegmenterBuilder::new()
    .in_periods(["。"])
    .parentheses([('「', '」')])
    .build();
let text = "私は「はい。そうです。」と答えた。";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["私は「はい。そうです。」と答えた。"];
assert_eq!(sentences, expected);
```

#### Words

```rust
use easy_segmenter::SegmenterBuilder;

let seg = SegmenterBuilder::new()
    .in_periods(["。"])
    .no_break_words(["モーニング娘。"])
    .build();
let text = "モーニング娘。の新曲";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["モーニング娘。の新曲"];
assert_eq!(sentences, expected);
```

#### Regex

```rust
use regex::Regex;
use easy_segmenter::SegmenterBuilder;

let seg = SegmenterBuilder::new()
    .in_periods(["．"])
    .no_break_regex(Regex::new(r"\d(．)\d").unwrap())
    .build();
let text = "３．１４";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["３．１４"];
assert_eq!(sentences, expected);
```


## Not supported by easy-segmenter

For simplicity, easy-segmenter does not support any function that requires editing of the original text,
although such functions are often supported by other tools.

### Normalization

The method of normalization depends on your application, and there are several possible methods.
Therefore, it should not be included in easy-segmenter.

### Fixing errant line breaks

Some other tools erase line breaks that are erroneously inserted in a sentence.

```
"新しい\n教科書" => ["新しい教科書"]
```

easy-segmenter does not fix such errors because whether or not it is an error depends on the data.

```
"新しい\n教科書" => ["新しい", "教科書"]
```

Those errors should be corrected using natural language processing techniques in pre- or post-processing.

### Quotation blocks

Quotation blocks are not also corrected in easy-segmenter.

```
> コーディングが好きなソフトウェ
> アエンジニアや研究が好きなリサ
> ーチエンジニアを募集しています
```