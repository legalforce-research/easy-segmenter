use std::io::Read;

use easy_segmenter::SegmenterBuilder;

fn main() {
    let runs = 10;
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();

    let mut num_sents = 0;
    let start = std::time::Instant::now();
    for _ in 0..runs {
        let seg = SegmenterBuilder::new()
            .in_delimiters(["。", "?", "!"])
            .ex_delimiters(["\n", "\r\n", "\r"])
            .quotes([('(', ')'), ('「', '」')])
            .build()
            .unwrap();
        num_sents += seg.segment(&text).count();
    }
    let duration = start.elapsed();

    println!(
        "easy_segmenter: {} ms, {} sentences",
        duration.as_secs_f64() * 1000. / runs as f64,
        num_sents / runs
    );
}
