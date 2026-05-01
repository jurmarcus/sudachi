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
