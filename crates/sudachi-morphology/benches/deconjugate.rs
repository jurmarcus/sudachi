//! Throughput benchmark for the bidirectional deconjugator.
//!
//! Measures end-to-end `deconjugate(input)` time on a representative
//! workload: a hand-picked set of conjugated forms covering every
//! verb / adjective / copula class. Reflects the real shape of work
//! done when called from a tokenizer pipeline (one verb at a time,
//! arbitrary surface forms in, candidate base forms out).
//!
//! Run with:
//!     cargo bench -p sudachi-morphology

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudachi_morphology::deconjugate;

/// Representative workload — 50 conjugated forms across every
/// conjugation class. Each is something a tokenizer might encounter
/// in real text.
const WORKLOAD: &[&str] = &[
    // V1 (Ichidan)
    "食べた",
    "食べない",
    "食べます",
    "食べました",
    "食べさせられる",
    "食べてしまった",
    "食べられる",
    "食べたくない",
    // V5K (Godan -ku)
    "書いた",
    "書かない",
    "書いて",
    "書きます",
    "書ける",
    "書いてる",
    // V5R (Godan -ru)
    "走った",
    "走らない",
    "走れば",
    "走ります",
    "走らせる",
    // V5M (Godan -mu)
    "読んだ",
    "読まない",
    "読みたい",
    "読まれる",
    // V5G (Godan -gu)
    "泳いだ",
    "泳がない",
    // V5U (Godan -u)
    "買った",
    "買わない",
    "買えば",
    // V5T (Godan -tsu)
    "持った",
    "持ちます",
    // V5S (Godan -su)
    "話した",
    "話して",
    // V5N (Godan -nu)
    "死んだ",
    // V5B (Godan -bu)
    "飛んだ",
    "遊んでる",
    // VK (Kuru)
    "来た",
    "来ない",
    "来られる",
    // VS-I (Suru)
    "した",
    "しない",
    "される",
    // V5K-S (Iku irregular)
    "行った",
    "行ってる",
    // V5R-I (Aru irregular)
    "あった",
    // Adj-i
    "高くない",
    "高かった",
    "高くて",
    "高ければ",
    // Cop
    "だった",
    "じゃない",
];

fn bench_default_workload(c: &mut Criterion) {
    c.bench_function("deconjugate/workload-50", |b| {
        b.iter(|| {
            for input in WORKLOAD {
                let _ = black_box(deconjugate(black_box(input)));
            }
        });
    });
}

fn bench_per_input_avg(c: &mut Criterion) {
    let mut group = c.benchmark_group("deconjugate/per-input");
    for input in &["食べた", "食べさせられる", "書いていない", "走らされた", "高くなかった"] {
        group.bench_function(*input, |b| {
            b.iter(|| {
                let _ = black_box(deconjugate(black_box(input)));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_default_workload, bench_per_input_avg);
criterion_main!(benches);
