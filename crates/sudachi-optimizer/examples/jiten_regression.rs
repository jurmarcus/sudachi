//! Run Sirius's 13 documented Sudachi failures through raw + optimised pipelines.
//! Usage: SUDACHI_DICT_PATH=~/.sudachi/.../system_full.dic
//!   cargo run --release --example jiten_regression -p sudachi-optimizer

use std::sync::Arc;
use sudachi_optimizer::{Optimizer, load_dictionary};

struct Case {
    label: &'static str,
    input: &'static str,
    expected: &'static [&'static str],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict = Arc::new(load_dictionary(&dict_path)?);
    let opt = Optimizer::new(dict);

    let cases = [
        Case { label: "01 compound noun overabsorption",   input: "少女の手によって",         expected: &["少女","の","手","によって"] },
        Case { label: "02 particle 'toshite' fusion",      input: "社長として",               expected: &["社長","として"] },
        Case { label: "03 aux verb orphaning (足蹴られた)", input: "足蹴られた",               expected: &["足","蹴られた"] },
        Case { label: "04 long vowel mark cross-boundary", input: "あなたーそこにいるの",     expected: &["あなた","そこ","に","いる","の"] },
        Case { label: "05 volitional + elongation",        input: "手伝って来るー",           expected: &["手伝って","来る"] },
        Case { label: "06 counter vs surname (何本)",      input: "何本ぐらいにしようかな",   expected: &["何","本","ぐらい","に","しよう","か","な"] },
        Case { label: "07 derogatory suffix め",           input: "欠陥品め",                 expected: &["欠陥品"] },
        Case { label: "08 idiom 手を抜く",                  input: "手を抜いているんですか",  expected: &["手を抜いている","んです","か"] },
        Case { label: "09 classical て-form",              input: "繕って貰いて",             expected: &["繕って","貰いて"] },
        Case { label: "10 fused interjection ね",          input: "ごめんなさいね",           expected: &["ごめんなさい","ね"] },
        Case { label: "11 false surname (いわね)",         input: "やっぱりまずいわね",       expected: &["やっぱり","まずい","わ","ね"] },
        Case { label: "12 expression resegmented (気がつく)", input: "気がついてしまう",     expected: &["気がついて","しまう"] },
        Case { label: "13 internal hiragana elongation",   input: "じゃなーい",               expected: &["じゃない"] },
    ];

    let mut raw_pass = 0;
    let mut opt_pass = 0;

    for c in &cases {
        let raw: Vec<String> = opt.tokenize_raw(c.input)?.iter().map(|m| m.surface.clone()).collect();
        let optimised: Vec<String> = opt.tokenize(c.input)?.iter().map(|m| m.surface.clone()).collect();

        let raw_ok = raw == c.expected;
        let opt_ok = optimised == c.expected;
        if raw_ok { raw_pass += 1; }
        if opt_ok { opt_pass += 1; }

        let status = match (raw_ok, opt_ok) {
            (true, true)   => "BOTH-OK",
            (false, true)  => "FIXED  ",
            (true, false)  => "REGRESS",
            (false, false) => "BROKEN ",
        };
        println!("[{status}] {label}", label = c.label);
        println!("    input    : {}", c.input);
        println!("    expected : {:?}", c.expected);
        println!("    raw      : {:?}", raw);
        println!("    optimised: {:?}", optimised);
        println!();
    }

    println!("=== summary ===");
    println!("raw       passes: {raw_pass}/{}", cases.len());
    println!("optimised passes: {opt_pass}/{}", cases.len());
    println!("net fixed by pipeline: {}", opt_pass as i32 - raw_pass as i32);
    Ok(())
}
