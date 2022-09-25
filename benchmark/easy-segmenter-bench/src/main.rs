use std::io::BufRead;

use easy_segmenter::Segmenter;

const RUNS: usize = 100;

fn main() {
    let mut text = String::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        text.push_str(&line);
        text.push_str("\n");
    }

    let seg = Segmenter::with_template_ja_config();

    let mut num_sents = 0;
    let start = std::time::Instant::now();
    for _ in 0..RUNS {
        num_sents += seg.segment(&text).count();
    }
    let duration = start.elapsed();

    println!(
        "easy_segmenter: {} ms, {} sentences",
        duration.as_secs_f64() * 1000. / RUNS as f64,
        num_sents / RUNS
    );
}
