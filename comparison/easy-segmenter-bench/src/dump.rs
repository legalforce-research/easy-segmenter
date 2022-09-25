use std::io::Read;

use easy_segmenter::SegmenterBuilder;

fn main() {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();

    let seg = SegmenterBuilder::new()
        .in_periods(["。", "?", "!"])
        .ex_periods(["\n", "\r\n", "\r"])
        .parentheses([('(', ')'), ('「', '」')])
        .build()
        .unwrap();
    for (i, j) in seg.segment(&text) {
        println!("{}", &text[i..j]);
    }
}
