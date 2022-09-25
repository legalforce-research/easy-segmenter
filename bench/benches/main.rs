use std::fs::File;
use std::io::Read;
use std::time::Duration;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, SamplingMode,
};

use easy_segmenter::SegmenterBuilder;

const SAMPLE_SIZE: usize = 10;
const WARM_UP_TIME: Duration = Duration::from_secs(5);
const MEASURE_TIME: Duration = Duration::from_secs(10);

fn criterion_waganeko(c: &mut Criterion) {
    let mut group = c.benchmark_group("waganeko");
    group.sample_size(SAMPLE_SIZE);
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASURE_TIME);
    group.sampling_mode(SamplingMode::Flat);

    let text = load_text("data/wagahaiwa_nekodearu.txt");
    add_segment_benches(&mut group, &text);
}

fn criterion_gakumon(c: &mut Criterion) {
    let mut group = c.benchmark_group("gakumon");
    group.sample_size(SAMPLE_SIZE);
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASURE_TIME);
    group.sampling_mode(SamplingMode::Flat);

    let text = load_text("data/gakumonno_susume.txt");
    add_segment_benches(&mut group, &text);
}

fn add_segment_benches(group: &mut BenchmarkGroup<WallTime>, text: &str) {
    group.bench_function("only-periods", |b| {
        b.iter(|| {
            let seg = SegmenterBuilder::new()
                .in_periods(["。", "？", "！"])
                .ex_periods(["\n", "\r\n", "\r"])
                .build()
                .unwrap();
            let dummy = seg.segment(&text).count();
            assert_ne!(dummy, 0);
        });
    });

    group.bench_function("with-parentheses", |b| {
        b.iter(|| {
            let seg = SegmenterBuilder::new()
                .in_periods(["。", "？", "！"])
                .ex_periods(["\n", "\r\n", "\r"])
                .parentheses([('(', ')'), ('「', '」')])
                .build()
                .unwrap();
            let dummy = seg.segment(&text).count();
            assert_ne!(dummy, 0);
        });
    });

    group.bench_function("with-regex", |b| {
        b.iter(|| {
            let seg = SegmenterBuilder::new()
                .in_periods(["。", "？", "！"])
                .ex_periods(["\n", "\r\n", "\r"])
                .no_break_regex(regex::Regex::new(r"(。{2,})。").unwrap())
                .build()
                .unwrap();
            let dummy = seg.segment(&text).count();
            assert_ne!(dummy, 0);
        });
    });
}

fn load_text(filename: &str) -> String {
    let mut text = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut text)
        .unwrap();
    text
}

criterion_group!(benches, criterion_waganeko, criterion_gakumon);
criterion_main!(benches);
