//! Tests for BucketingConfig — the knob that lets the bench harness
//! override the hardcoded 4-bucket-when-n>=8 heuristic in
//! Pipeline::parse_morphemes.

use sudachi_kwja::BucketingConfig;

#[test]
fn default_uses_heuristic() {
    let cfg = BucketingConfig::default();
    assert_eq!(cfg.num_buckets, None, "default = heuristic, not forced");
    assert_eq!(
        cfg.min_chunks_for_bucketing, 8,
        "matches existing n>=8 check"
    );
}

#[test]
fn force_one_bucket() {
    let cfg = BucketingConfig {
        num_buckets: Some(1),
        ..Default::default()
    };
    assert_eq!(cfg.num_buckets, Some(1));
}

#[test]
fn force_n_buckets() {
    for n in [2_usize, 4, 8, 16] {
        let cfg = BucketingConfig {
            num_buckets: Some(n),
            ..Default::default()
        };
        assert_eq!(cfg.num_buckets, Some(n));
    }
}

#[test]
fn config_is_copy() {
    fn assert_copy<T: Copy>() {}
    assert_copy::<BucketingConfig>();
}

// Behavioral tests for the bucket-count math. These don't load a real
// model checkpoint — bucketing logic is independent of GPU work.

#[test]
fn default_with_few_chunks_uses_one_bucket() {
    let cfg = BucketingConfig::default();
    assert_eq!(
        sudachi_kwja::pipeline::compute_bucket_count(5, cfg),
        1,
        "n=5 < 8 threshold → 1 bucket"
    );
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(7, cfg), 1);
}

#[test]
fn default_with_enough_chunks_uses_four_buckets() {
    let cfg = BucketingConfig::default();
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(8, cfg), 4);
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(100, cfg), 4);
}

#[test]
fn force_one_bucket_overrides_heuristic() {
    let cfg = BucketingConfig {
        num_buckets: Some(1),
        ..Default::default()
    };
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(100, cfg), 1);
}

#[test]
fn force_eight_buckets_caps_at_n_chunks() {
    let cfg = BucketingConfig {
        num_buckets: Some(8),
        ..Default::default()
    };
    // Force 8 buckets but only 3 chunks: cap at n_chunks.
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(3, cfg), 3);
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(100, cfg), 8);
}

#[test]
fn lower_threshold_skips_heuristic_floor() {
    let cfg = BucketingConfig {
        num_buckets: None,
        min_chunks_for_bucketing: 4,
    };
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(4, cfg), 4);
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(3, cfg), 1);
}

#[test]
fn zero_chunks_returns_one_bucket() {
    // Edge case: zero items. Bucket count must still be ≥ 1 to avoid
    // division-by-zero in the bucketing loop.
    let cfg = BucketingConfig::default();
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(0, cfg), 1);
    let forced = BucketingConfig {
        num_buckets: Some(8),
        ..Default::default()
    };
    assert_eq!(sudachi_kwja::pipeline::compute_bucket_count(0, forced), 1);
}
