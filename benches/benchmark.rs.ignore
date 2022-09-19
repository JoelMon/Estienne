use criterion::{black_box, criterion_group, criterion_main, Criterion};
use est::{is_scripture, Locale};

fn is_scripture_bench(c: &mut Criterion) {
    let scriptures = black_box([
        "Matthew 24:14",
        "John 3:16",
        "Psalms 94:19",
        "Psalms 17:17",
        "Revelations 1:5",
        "Exodus 1:1",
        "1 John 2:4",
        "Jude 2:10",
    ]);

    c.bench_function("is_scripture", |b| {
        est::Locale::new(Locale::en_us);
        b.iter(|| est::is_scripture(scriptures.iter().next().unwrap()))
    });
}

criterion_group!(benches, is_scripture_bench);
criterion_main!(benches);
