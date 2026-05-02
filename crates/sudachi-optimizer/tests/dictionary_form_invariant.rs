//! Invariant test for [`Morpheme::dictionary_form`].
//!
//! Locks in the contract that downstream consumers (jisho-core, etc.)
//! depend on: after the full optimizer pipeline, every verb morpheme's
//! `dictionary_form` is a valid lemma — never the merged surface — even
//! when [`combine`](sudachi_optimizer::combine) or
//! [`repair`](sudachi_optimizer::repair) stages have rewritten the
//! surface and `normalized_form`.
//!
//! Was a long diagnostic dump; now an executable assertion. Run with:
//! `cargo test -p sudachi-optimizer --test dictionary_form_invariant`.

use std::path::PathBuf;
use std::sync::Arc;

use sudachi_optimizer::{Optimizer, load_dictionary};

fn dict_path() -> PathBuf {
    if let Ok(p) = std::env::var("SUDACHI_DICT_PATH") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").unwrap();
    let candidates = [
        format!("{home}/.local/share/sudachi/sudachi-dictionary-20260428/system_full.dic"),
        format!("{home}/.local/share/sudachi/sudachi-dictionary-20260116/system_full.dic"),
        format!("{home}/.local/share/sudachi/sudachi-dictionary-20240409/system_full.dic"),
    ];
    for c in candidates {
        let p = PathBuf::from(&c);
        if p.exists() {
            return p;
        }
    }
    let dir = format!("{home}/.local/share/sudachi");
    if let Ok(read) = std::fs::read_dir(&dir) {
        for e in read.flatten() {
            let path = e.path();
            if let Some(name) = path.file_name().and_then(|s| s.to_str())
                && name.starts_with("sudachi-dictionary-")
            {
                let dic = path.join("system_full.dic");
                if dic.exists() {
                    return dic;
                }
            }
        }
    }
    panic!("no Sudachi dictionary found; set SUDACHI_DICT_PATH");
}

fn opt() -> Optimizer {
    let dict = load_dictionary(&dict_path()).expect("load dict");
    Optimizer::new(Arc::new(dict))
}

/// After the full pipeline, godan ん-stem te/ta merges keep the
/// **lemma** in `dictionary_form` while writing the merged surface to
/// `surface` and `normalized_form`. Locks the invariant against
/// regression in [`combine::inflections`](sudachi_optimizer::combine)
/// and [`repair::n_tokenisation`](sudachi_optimizer::repair).
#[test]
fn merged_godan_n_stem_keeps_lemma_in_dictionary_form() {
    let cases = [
        ("飲んだ", "飲む"),
        ("飲んで", "飲む"),
        ("読んだ", "読む"),
        ("住んでいる", "住む"),
        ("食べている", "食べる"),
        ("なった", "なる"),
        ("飲んだら", "飲む"),
        ("及んだ", "及ぶ"),
        ("産んだ", "産む"),
        ("並んで", "並ぶ"),
        ("死んだ", "死ぬ"),
    ];
    let optimizer = opt();
    for (input, expected_lemma) in cases {
        let ms = optimizer.tokenize(input).expect("tokenize");
        let head = &ms[0];
        assert_eq!(
            head.dictionary_form, expected_lemma,
            "{input}: expected dict_form={expected_lemma}, got dict={} \
             (surface={}, normalized={}). Merge stages must preserve the \
             lemma in dictionary_form even when normalized_form is \
             rewritten to the merged surface.",
            head.dictionary_form, head.surface, head.normalized_form
        );
    }
}
