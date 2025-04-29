#[macro_use]
extern crate criterion;

use criterion::Criterion;
use lararust_i18n::{set_language, lang, __};


fn bench_lookup_only(c: &mut Criterion) {
    set_language("en");
    let _ = crate::lang::TRANSLATION_REGISTRY.get("en").unwrap();
    
    c.bench_function("simple lookup no replace", |b| {
        b.iter(|| crate::lang::TRANSLATION_REGISTRY
            .get("en")
            .unwrap()
            .get("menu.quit")
            .unwrap())
    });
}

fn bench_language_switch(c: &mut Criterion) {
    c.bench_function("switch languages", |b| {
        b.iter(|| {
            set_language("en");
            let _ = __("_", &[]);
            set_language("es");
            let _ = __("_", &[]);
        });
    });
}

criterion_group!(benches, bench_lookup_only, bench_language_switch);
criterion_main!(benches);