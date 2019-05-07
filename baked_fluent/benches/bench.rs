use baked_fluent::{impl_localize, localize, Localize};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

impl_localize! {
    #[localize(path = "test-i18n", default_locale = "en_US")]
    struct TestLocalizer(_);
}

fn bench(c: &mut Criterion) {
    c.bench_function("negotiate-fast", |b| {
        b.iter(|| {
            black_box(TestLocalizer::new(black_box(&["en_US"]), black_box(None)));
        });
    });

    c.bench_function("negotiate-slow", |b| {
        b.iter(|| {
            black_box(TestLocalizer::new(
                black_box(&["zh_HK"]),
                black_box(Some("es_MX,es,en_UK,en_AU,en_US,en;q=0.5")),
            ));
        });
    });
    c.bench_function("localize-simple", |b| {
        let loc = TestLocalizer::new(&["en_US"], None);

        b.iter(|| {
            black_box(localize!(loc, title));
        });
    });
    c.bench_function("localize-moderate", |b| {
        let loc = TestLocalizer::new(&["en_US"], None);

        b.iter(|| {
            black_box(localize!(
                loc,
                greeting,
                name = ("Jamie"),
                friends = black_box(12)
            ));
        });
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
