use super::*;

#[test]
fn test_simple_1() {
    let seg = SegmenterBuilder::new().in_periods(["。"]).build();
    let text = "これはペンです。それはマーカーです。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["これはペンです。", "それはマーカーです。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_simple_2() {
    let seg = SegmenterBuilder::new().in_periods(["。", "？"]).build();
    let text = "それは何ですか？ペンです。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["それは何ですか？", "ペンです。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_simple_3() {
    let seg = SegmenterBuilder::new()
        .in_periods(["！"])
        .ex_periods(["\n"])
        .build();
    let text = "良かったね\nすごい！";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["良かったね", "すごい！"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_simple_4() {
    let seg = SegmenterBuilder::new().ex_periods(["\n", "</br>"]).build();
    let text = "良かったね</br>すごい\n";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["良かったね", "すごい"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_duplicate_periods() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。", "。。。"])
        .ex_periods(["\n", "\r\n", "\r"])
        .build();
    let text = "なるほど。。。その通りですね\r\n";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["なるほど。。。", "その通りですね"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_listing() {
    let seg = SegmenterBuilder::new().ex_periods(["\n", "\n・"]).build();
    let text = "買うもの\n・ご飯\n・卵\n・醤油\n計３点";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["買うもの", "ご飯", "卵", "醤油", "計３点"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_quote_1() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .parentheses([('「', '」')])
        .build();
    let text = "私は「はい。そうです。」と答えた。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["私は「はい。そうです。」と答えた。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_quote_2() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .parentheses([('（', '）')])
        .build();
    let text = "私は「はい。そうです。」と答えた。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["私は「はい。", "そうです。", "」と答えた。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_quote_3() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .parentheses([('「', '」'), ('（', '）')])
        .build();
    let text = "私は「はい。そうです。（嘘だけど。）」と答えた。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["私は「はい。そうです。（嘘だけど。）」と答えた。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_quote_4() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .parentheses([('「', '」'), ('（', '）')])
        .build();
    let text = "私は「はい。そうです。（嘘だけど。）と答えた。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["私は「はい。", "そうです。", "（嘘だけど。）と答えた。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_word_1() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .no_break_words(["モーニング娘。"])
        .build();
    let text = "モーニング娘。の新曲。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["モーニング娘。の新曲。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_regex_1() {
    let seg = SegmenterBuilder::new()
        .in_periods(["．"])
        .no_break_regex(Regex::new(r"\d(．)\d").unwrap())
        .build();
    let text = "３．１４１５９２．円周率です．";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["３．１４１５９２．", "円周率です．"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_regex_2() {
    let seg = SegmenterBuilder::new()
        .in_periods(["。"])
        .no_break_regex(Regex::new(r"(。{2,})。").unwrap())
        .build();
    let text = "はぁ。。。疲れた。。";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["はぁ。。。", "疲れた。", "。"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_no_last_period() {
    let seg = SegmenterBuilder::new().in_periods(["。"]).build();
    let text = "これはペンです。それはマーカーです";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec!["これはペンです。", "それはマーカーです"];
    assert_eq!(sentences, expected);
}

#[test]
fn test_empty_text() {
    let seg = SegmenterBuilder::new().in_periods(["。"]).build();
    let text = "";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    assert!(sentences.is_empty());
}

#[test]
fn test_basic() {
    let seg = Segmenter::with_basic_configure();
    let text = "円周率はいくつですか？３．１４です。なるほど、\
        以前に「３の方が良いのでは？」と聞いた気がしますが\n今も３．１４なんですね";
    let sentences: Vec<_> = seg.segment(text).map(|(i, j)| &text[i..j]).collect();
    let expected = vec![
        "円周率はいくつですか？",
        "３．１４です。",
        "なるほど、以前に「３の方が良いのでは？」と聞いた気がしますが",
        "今も３．１４なんですね",
    ];
    assert_eq!(sentences, expected);
}
