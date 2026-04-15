use std::sync::Arc;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi_search::SearchTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args().nth(1).unwrap_or_else(|| "時々港まで散歩します。".to_string());

    // Load dictionary
    let dict_path = std::env::var("SUDACHI_DICT_PATH")?;
    let dict_bytes = std::fs::read(&dict_path)?;
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)?;

    let tokenizer = SearchTokenizer::new(Arc::new(dictionary));

    println!("Input: {}", input);
    println!();

    // Normalized (default)
    println!("=== Normalized Form ===");
    let tokens = tokenizer.tokenize(&input)?;
    for token in &tokens {
        let colocated = if token.is_colocated { " (colocated)" } else { "" };
        println!("  {:12} [{:2}-{:2}]{}", token.surface, token.byte_start, token.byte_end, colocated);
    }

    // Surface form
    println!();
    println!("=== Surface Form ===");
    let tokens = tokenizer.tokenize_with_normalization(&input, false)?;
    for token in &tokens {
        let colocated = if token.is_colocated { " (colocated)" } else { "" };
        println!("  {:12} [{:2}-{:2}]{}", token.surface, token.byte_start, token.byte_end, colocated);
    }

    // Compounds
    println!();
    println!("=== Compound Words ===");
    let compounds = tokenizer.detect_compounds(&input)?;
    if compounds.is_empty() {
        println!("  (no compounds detected)");
    } else {
        for c in &compounds {
            println!("  {} = {:?}", c.surface, c.components);
        }
    }

    Ok(())
}
