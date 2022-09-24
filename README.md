# easy-segmenter

Fast and customizable, but easy-to-use, rule-based sentence segmenter.

## Basic usage

## Customizability

`easy_segmenter::Segmenter` does not hardcode any segmentation rules and
provides a simple framework of rule definitions.

You only need to define the two types of rules:
- what to segment sentences by, and
- what not to segment.

### Periods for segmenting sentences

easy-segmenter handles two types of sentence terminators (or *periods*):

- *Inclusive periods* that are included in resulting sentences, and
- *Exclusive periods* that are excluded in resulting sentences.

```rust
use easy_segmenter::SegmenterBuilder;

let seg = SegmenterBuilder::new()
    .in_periods(["。"]) // Inclusive periods
    .ex_periods(["\n"]) // Exclusive periods
    .build();
let text = "なるほど\nその通りですね。";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["なるほど", "その通りですね。"]; // "\n" is excluded.
assert_eq!(sentences, expected);
```

Periods are detected with exact string matching for a set of patterns.
If multiple periods are overlapped at a position,
the [leftmost-longest one](https://docs.rs/aho-corasick/latest/aho_corasick/enum.MatchKind.html#variant.LeftmostLongest) is detected.
The match semantics allow for handling specific cases such as carriage returns and multiple dots.

```rust
let seg = SegmenterBuilder::new()
    .in_periods(["。", "。。。"])
    .ex_periods(["\n", "\r\n", "\r"])
    .build();
let text = "なるほど。。。その通りですね\r\n";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["なるほど。。。", "その通りですね"];
assert_eq!(sentences, expected);
```

Itemization can be also handled.

```rust
let seg = SegmenterBuilder::new().ex_periods(["\n", "\n・"]).build();
let text = "買うもの\n・ご飯\n・卵\n・醤油";
let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
let expected = vec!["買うもの", "ご飯", "卵", "醤油"];
assert_eq!(sentences, expected);
```

### Rules for not segmenting sentences

easy-segmenter

#### Quotation

Quoted sentences will not be segmented.
You can define pairs of parentheses to specify quotations.

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

You can define words that should not be segmented.

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

You can define regex patterns that should not be segmented.
Captured patterns will not be segmented.

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

Regular expressions are powerful, but complicated ones can slow down segmentation.
Consider using `no_break_words` first to solve your problem.

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
>> コーディングが好きなソフトウェ
>> アエンジニアや研究が好きなリサ
>> ーチエンジニアを募集しています
```