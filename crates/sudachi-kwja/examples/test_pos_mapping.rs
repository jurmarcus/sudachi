fn main() {
    // Test that the binary actually maps Sudachi POS to KWJA POS.
    use kwja::pipeline::sudachi_to_kwja_pos;
    let cases = [
        ("補助記号", "特殊"),
        ("形状詞", "形容詞"),
        ("名詞", "名詞"),
        ("代名詞", "指示詞"),
    ];
    for (input, expected) in &cases {
        let actual = sudachi_to_kwja_pos(input);
        println!("{} -> {} (expected {})", input, actual, expected);
        assert_eq!(actual, *expected);
    }
    println!("ALL OK");
}
