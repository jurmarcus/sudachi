//! Token stream implementation for Sudachi.

use tantivy_tokenizer_api::{Token, TokenStream};

use crate::tokenizer::TokenData;

/// A token stream wrapping pre-extracted Sudachi tokens.
///
/// Implements Tantivy's `TokenStream` trait to iterate over
/// tokens extracted from Sudachi morphemes.
///
/// For B+C multi-granularity (Search mode), colocated tokens share the same
/// position as the previous token, enabling both compound word and sub-token
/// matching at the same document location.
pub struct SudachiTokenStream<'a> {
    /// Pre-extracted tokens
    tokens: Vec<TokenData>,
    /// Current token (shared, mutable)
    token: &'a mut Token,
    /// Current index in token list
    index: usize,
    /// Current position (incremented only for non-colocated tokens)
    position: usize,
}

impl<'a> SudachiTokenStream<'a> {
    /// Create a new token stream.
    pub(crate) fn new(tokens: Vec<TokenData>, token: &'a mut Token) -> Self {
        Self {
            tokens,
            token,
            index: 0,
            position: 0,
        }
    }
}

impl TokenStream for SudachiTokenStream<'_> {
    fn advance(&mut self) -> bool {
        if self.index >= self.tokens.len() {
            return false;
        }

        let token_data = &self.tokens[self.index];

        // Set token text
        self.token.text.clear();
        self.token.text.push_str(&token_data.text);

        // Sudachi returns byte offsets directly (no conversion needed)
        self.token.offset_from = token_data.offset_from;
        self.token.offset_to = token_data.offset_to;

        // Position handling for colocated tokens:
        // - Colocated tokens share the same position as the previous token
        // - Non-colocated tokens get a new position
        if !token_data.is_colocated {
            // New position for primary tokens
            if self.index > 0 {
                self.position += 1;
            }
        }
        // Colocated tokens keep the same position (don't increment)

        self.token.position = self.position;
        self.token.position_length = 1;

        self.index += 1;
        true
    }

    #[inline(always)]
    fn token(&self) -> &Token {
        self.token
    }

    #[inline(always)]
    fn token_mut(&mut self) -> &mut Token {
        self.token
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here but require a loaded dictionary
}
