//! Shows ALL morphological information Sudachi provides for each token.
//!
//! Usage: SUDACHI_DICT_PATH=~/.sudachi/.../system.dic cargo run --example full_info -- "此処が僕の家。"

use std::sync::Arc;
use sudachi::analysis::Tokenize;
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi::prelude::Mode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "此処が僕の家。".to_string());

    // Load dictionary
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict_bytes = std::fs::read(&dict_path)?;
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
    let dictionary = Arc::new(JapaneseDictionary::from_cfg_storage_with_embedded_chardef(
        &config, storage,
    )?);

    let tokenizer = StatelessTokenizer::new(dictionary);

    println!("Input: {}", input);
    println!(
        "Length: {} bytes, {} chars",
        input.len(),
        input.chars().count()
    );
    println!();

    // Mode C (coarsest - compounds preserved)
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           MODE C (Coarsest)                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    print_morphemes(&tokenizer, &input, Mode::C)?;

    // Mode B (medium)
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           MODE B (Medium)                                    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    print_morphemes(&tokenizer, &input, Mode::B)?;

    // Mode A (finest)
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           MODE A (Finest)                                    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    print_morphemes(&tokenizer, &input, Mode::A)?;

    Ok(())
}

fn print_morphemes(
    tokenizer: &StatelessTokenizer<Arc<JapaneseDictionary>>,
    input: &str,
    mode: Mode,
) -> Result<(), Box<dyn std::error::Error>> {
    let morphemes = tokenizer.tokenize(input, mode, false)?;

    for (i, m) in morphemes.iter().enumerate() {
        // Get word info - need to specify type for get_word_info
        let word_info = sudachi::analysis::morpheme::Morpheme::get_word_info(&m);

        println!();
        println!("┌─────────────────────────────────────────────────────────────────────────────┐");
        println!(
            "│ Token #{:<3}                                                                 │",
            i
        );
        println!("├─────────────────────────────────────────────────────────────────────────────┤");

        // Surface and forms
        println!("│ Surface:          {:58}│", format!("\"{}\"", m.surface()));
        println!(
            "│ Dictionary Form:  {:58}│",
            format!("\"{}\"", m.dictionary_form())
        );
        println!(
            "│ Normalized Form:  {:58}│",
            format!("\"{}\"", m.normalized_form())
        );
        println!(
            "│ Reading Form:     {:58}│",
            format!("\"{}\"", m.reading_form())
        );

        println!("├─────────────────────────────────────────────────────────────────────────────┤");

        // Part of speech (品詞)
        let pos = m.part_of_speech();
        let pos_str = pos.join(" / ");
        // Handle long POS strings
        if pos_str.len() <= 58 {
            println!("│ Part of Speech:   {:58}│", pos_str);
        } else {
            println!(
                "│ Part of Speech:                                                             │"
            );
            for chunk in pos_str.as_bytes().chunks(70) {
                let s = String::from_utf8_lossy(chunk);
                println!("│   {:72}│", s);
            }
        }
        println!("│ POS ID:           {:58}│", m.part_of_speech_id());

        println!("├─────────────────────────────────────────────────────────────────────────────┤");

        // Positions
        println!(
            "│ Byte Range:       [{:3} - {:3}]                                               │",
            m.begin(),
            m.end()
        );
        println!(
            "│ Char Range:       [{:3} - {:3}]                                               │",
            m.begin_c(),
            m.end_c()
        );

        println!("├─────────────────────────────────────────────────────────────────────────────┤");

        // Word metadata
        println!("│ Word ID:          {:58}│", format!("{:?}", m.word_id()));
        println!("│ Dictionary ID:    {:58}│", m.dictionary_id());
        println!("│ Is OOV:           {:58}│", m.is_oov());
        println!("│ Total Cost:       {:58}│", m.total_cost());

        // Splits (A and B unit information)
        let a_split = word_info.a_unit_split();
        let b_split = word_info.b_unit_split();
        let word_struct = word_info.word_structure();

        if !a_split.is_empty() || !b_split.is_empty() || !word_struct.is_empty() {
            println!(
                "├─────────────────────────────────────────────────────────────────────────────┤"
            );
            if !a_split.is_empty() {
                println!("│ A-Unit Split:     {:58}│", format!("{:?}", a_split));
            }
            if !b_split.is_empty() {
                println!("│ B-Unit Split:     {:58}│", format!("{:?}", b_split));
            }
            if !word_struct.is_empty() {
                println!("│ Word Structure:   {:58}│", format!("{:?}", word_struct));
            }
        }

        // Synonyms
        let synonyms = m.synonym_group_ids();
        if !synonyms.is_empty() {
            println!(
                "├─────────────────────────────────────────────────────────────────────────────┤"
            );
            println!("│ Synonym Groups:   {:58}│", format!("{:?}", synonyms));
        }

        println!("└─────────────────────────────────────────────────────────────────────────────┘");
    }

    Ok(())
}
