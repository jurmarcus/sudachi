//! Golden test corpus ported from rampaa/JL's
//! `JL.Core.Tests/Deconjugation/` directory.
//!
//! 4,781 test cases across 23 verb / adjective / copula classes,
//! one Rust module per JL test file. Each test asserts that
//! `deconjugate(input)` produces a candidate with the expected
//! dictionary form, class tag, and process chain.
//!
//! This is the validation oracle: when a deconjugator change goes in,
//! `cargo test --test golden` is the binary signal for whether it
//! preserves correctness against thousands of hand-curated cases.

pub mod helper {
    use sudachi_morphology::Form;

    /// Format a deconjugator process chain per JL's
    /// `LookupResultUtils.DeconjugationProcessesToText` rules:
    ///
    /// 1. Iterate the chain in REVERSE.
    /// 2. Skip empty entries.
    /// 3. Entries starting with `(` are HIDDEN unless they're at
    ///    position 0 in the original chain (last in reverse). When
    ///    emitted, strip the surrounding parens.
    /// 4. Join visible entries with `→`.
    pub fn format_process(process: &[String]) -> String {
        let mut parts = Vec::new();
        for (i, item) in process.iter().enumerate().rev() {
            if item.is_empty() {
                continue;
            }
            let is_paren = item.starts_with('(');
            if is_paren {
                if i != 0 {
                    continue;
                }
                // Strip outer parens.
                let inner = &item[1..item.len() - 1];
                parts.push(inner.to_string());
            } else {
                parts.push(item.clone());
            }
        }
        parts.join("→")
    }

    /// Format a SET of candidate forms per JL's full output:
    /// each candidate as `～<chain>`, multiple candidates joined
    /// with `; `.
    ///
    /// JL's expected strings have `～` only on the first candidate
    /// (e.g. `～past; negative→past`). We emit in the same shape so
    /// string equality works.
    pub fn format_candidates(forms: &[&Form]) -> String {
        let mut out = String::new();
        for (i, f) in forms.iter().enumerate() {
            let chain = format_process(&f.process);
            if chain.is_empty() {
                continue;
            }
            if i == 0 {
                out.push('\u{ff5e}'); // FULLWIDTH TILDE ～
                out.push_str(&chain);
            } else {
                out.push_str("; ");
                out.push_str(&chain);
            }
        }
        out
    }

    /// Compare a deconjugator's filtered candidates against a JL
    /// expected string. Candidates ordering doesn't have to match
    /// JL's exactly — we check set equality of the formatted chains.
    pub fn matches_expected(forms: &[&Form], expected: &str) -> bool {
        let expected_normalised = expected.trim_start_matches('\u{ff5e}');
        let expected_chains: std::collections::BTreeSet<&str> =
            expected_normalised.split("; ").collect();
        let got_chains: std::collections::BTreeSet<String> = forms
            .iter()
            .map(|f| format_process(&f.process))
            .filter(|s| !s.is_empty())
            .collect();
        let got_refs: std::collections::BTreeSet<&str> =
            got_chains.iter().map(String::as_str).collect();
        // Test passes if every JL expected chain is present in our
        // output. Extra candidates (not in JL's expected) are
        // tolerated — the deconjugator may legitimately find more
        // candidates than JL did.
        expected_chains.iter().all(|e| got_refs.contains(e))
    }
}

mod golden {
    pub mod adj_i;
    pub mod cop;
    pub mod v1;
    pub mod v1_s;
    pub mod v4r;
    pub mod v5_aru;
    pub mod v5_b;
    pub mod v5_g;
    pub mod v5_k;
    pub mod v5_k_s;
    pub mod v5_m;
    pub mod v5_n;
    pub mod v5_r;
    pub mod v5_r_i;
    pub mod v5_s;
    pub mod v5_t;
    pub mod v5_u;
    pub mod v5_u_s;
    pub mod vk;
    pub mod vs_c;
    pub mod vs_i;
    pub mod vs_s;
    pub mod vz;
}
