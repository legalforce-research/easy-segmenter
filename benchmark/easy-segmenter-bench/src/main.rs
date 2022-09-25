use std::io::Read;

use easy_segmenter::Segmenter;

fn main() {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();

    let mut num_sents = 0;
    let start = std::time::Instant::now();
    for _ in 0..100 {
        num_sents += Segmenter::with_template_ja_config().segment(&text).count();
    }
    let duration = start.elapsed();

    println!(
        "easy_segmenter: {} ms, {} sentences",
        duration.as_secs_f64() * 1000. / 100.,
        num_sents / 100
    );
}
