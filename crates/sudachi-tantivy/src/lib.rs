// sudachi-tantivy: Tantivy tokenizer adapter for sudachi-search
//
// Translates SearchToken from sudachi-search into Tantivy's token stream:
//   is_colocated: false  → position_increment = 1  (new position)
//   is_colocated: true   → position_increment = 0  (same position, colocated)
//
// TODO: Implement tantivy::tokenizer::Tokenizer trait
// See: https://docs.rs/tantivy/latest/tantivy/tokenizer/index.html
