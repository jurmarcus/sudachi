//! Integration tests for sudachi-search.
//!
//! These tests require a Sudachi dictionary to be available.
//! Run with: SUDACHI_DICT_PATH=/path/to/system.dic cargo test -- --ignored

use std::sync::Arc;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::storage::{Storage, SudachiDicData};
use sudachi_search::SearchTokenizer;

/// Helper to load the dictionary from SUDACHI_DICT_PATH
fn load_tokenizer() -> SearchTokenizer {
    let dict_path = std::env::var("SUDACHI_DICT_PATH")
        .expect("SUDACHI_DICT_PATH must be set for integration tests");
    let dict_bytes = std::fs::read(&dict_path).expect("Failed to read dictionary file");
    let storage = SudachiDicData::new(Storage::Owned(dict_bytes));
    let config = Config::minimal_at(std::path::Path::new(&dict_path).parent().unwrap());
    let dictionary = JapaneseDictionary::from_cfg_storage_with_embedded_chardef(&config, storage)
        .expect("Failed to load dictionary");
    SearchTokenizer::new(Arc::new(dictionary))
}

// ============================================================================
// Basic Tokenization Tests
// ============================================================================

#[test]
#[ignore]
fn test_simple_verb() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("食べる").unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].surface, "食べる");
    assert!(!tokens[0].is_colocated);
}

#[test]
#[ignore]
fn test_compound_word() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("東京都立大学").unwrap();

    // Should have compound + subwords
    assert!(tokens.len() > 1, "Should emit multiple tokens for compound");

    // First token should be the full compound
    assert_eq!(tokens[0].surface, "東京都立大学");
    assert!(!tokens[0].is_colocated);

    // Subsequent tokens should be colocated subwords
    let colocated: Vec<_> = tokens.iter().filter(|t| t.is_colocated).collect();
    assert!(!colocated.is_empty(), "Should have colocated subwords");

    // Should include 大学 as a subword
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(surfaces.contains(&"大学"), "Should include 大学 subword");
}

// ============================================================================
// Function Word Filtering Tests
// ============================================================================

#[test]
#[ignore]
fn test_filter_past_tense_ta() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("食べた").unwrap();

    // With filtering (default), should only get 食べる
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"食べる"),
        "Should include normalized 食べる"
    );
    assert!(!surfaces.contains(&"た"), "Should filter out た");
}

#[test]
#[ignore]
fn test_filter_progressive_teiru() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("食べている").unwrap();

    // With filtering, should only get 食べる (て, いる filtered)
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"食べる"),
        "Should include normalized 食べる"
    );
    assert!(!surfaces.contains(&"て"), "Should filter out て");
    assert!(
        !surfaces.contains(&"いる"),
        "Should filter out いる (non-independent)"
    );
}

#[test]
#[ignore]
fn test_filter_adjective_past() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("美しかった").unwrap();

    // With filtering, should get 美しい (た filtered)
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"美しい"),
        "Should include normalized 美しい"
    );
    assert!(!surfaces.contains(&"た"), "Should filter out た");
}

#[test]
#[ignore]
fn test_filter_polite_masu() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("食べます").unwrap();

    // With filtering, should get 食べる (ます filtered)
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"食べる"),
        "Should include normalized 食べる"
    );
    assert!(!surfaces.contains(&"ます"), "Should filter out ます");
}

#[test]
#[ignore]
fn test_keep_case_particles() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("本を読む").unwrap();

    // Case particles like を should NOT be filtered
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(surfaces.contains(&"を"), "Should keep case particle を");
}

// ============================================================================
// with_all_tokens() Tests
// ============================================================================

#[test]
#[ignore]
fn test_with_all_tokens_preserves_ta() {
    let tokenizer = load_tokenizer().with_all_tokens();
    let tokens = tokenizer.tokenize("食べた").unwrap();

    // With all tokens, should include both 食べる and た
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(surfaces.contains(&"食べる"), "Should include 食べる");
    assert!(
        surfaces.contains(&"た"),
        "Should include た with all_tokens"
    );
}

#[test]
#[ignore]
fn test_with_all_tokens_preserves_teiru() {
    let tokenizer = load_tokenizer().with_all_tokens();
    let tokens = tokenizer.tokenize("食べている").unwrap();

    // With all tokens, should include て and いる (normalized to 居る)
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(surfaces.contains(&"食べる"), "Should include 食べる");
    assert!(
        surfaces.contains(&"て"),
        "Should include て with all_tokens"
    );
    // Note: いる normalizes to 居る (kanji form)
    assert!(
        surfaces.contains(&"居る"),
        "Should include 居る (normalized いる) with all_tokens"
    );
}

// ============================================================================
// Normalization Tests
// ============================================================================

#[test]
#[ignore]
fn test_normalized_form_default() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("食べた").unwrap();

    // Default is normalized form
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"食べる"),
        "Should normalize 食べ to 食べる"
    );
}

#[test]
#[ignore]
fn test_surface_form() {
    let tokenizer = load_tokenizer().with_surface_form().with_all_tokens();
    let tokens = tokenizer.tokenize("食べた").unwrap();

    // With surface form, should keep 食べ not 食べる
    let surfaces: Vec<_> = tokens.iter().map(|t| t.surface.as_str()).collect();
    assert!(
        surfaces.contains(&"食べ"),
        "Should use surface form 食べ, not normalized 食べる"
    );
}

#[test]
#[ignore]
fn test_hiragana_normalization() {
    let tokenizer = load_tokenizer();
    let tokens = tokenizer.tokenize("たべる").unwrap();

    // Hiragana should normalize to kanji
    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0].surface, "食べる",
        "Should normalize hiragana to kanji"
    );
}

// ============================================================================
// Compound Detection Tests
// ============================================================================

#[test]
#[ignore]
fn test_detect_compounds() {
    let tokenizer = load_tokenizer();
    let compounds = tokenizer.detect_compounds("東京都立大学で研究").unwrap();

    // Should detect 東京都立大学 as a compound
    assert!(!compounds.is_empty(), "Should detect at least one compound");

    let compound = &compounds[0];
    assert_eq!(compound.surface, "東京都立大学");
    assert!(compound.is_compound());
    assert!(compound.components.len() > 1);
}

#[test]
#[ignore]
fn test_no_compounds_in_simple_sentence() {
    let tokenizer = load_tokenizer();
    let compounds = tokenizer.detect_compounds("本を読む").unwrap();

    // Simple sentence should have no compounds
    assert!(
        compounds.is_empty() || compounds.iter().all(|c| !c.is_compound()),
        "Simple sentence should have no true compounds"
    );
}
