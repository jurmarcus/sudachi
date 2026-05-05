fn main() -> anyhow::Result<()> {
    let cp = sudachi_kwja::Checkpoint::load(&std::path::PathBuf::from(std::env::var("HOME")?).join(".local/share/jisho/checkpoints/word.safetensors"))?;
    let mut names = cp.tensor_names();
    names.sort();
    for n in &names[..30] { println!("{n}"); }
    println!("... ({} total)", names.len());
    println!();
    println!("=== encoder.* sample ===");
    for n in names.iter().filter(|n| n.starts_with("encoder.")).take(20) { println!("{n}"); }
    println!();
    println!("=== pos_tagger detail ===");
    for n in names.iter().filter(|n| n.starts_with("pos_tagger.")) {
        println!("  {n} -> {:?}", cp.get(n)?.dims());
    }
    println!();
    println!("=== word_feature_tagger detail ===");
    for n in names.iter().filter(|n| n.starts_with("word_feature_tagger.")) {
        println!("  {n} -> {:?}", cp.get(n)?.dims());
    }
    println!();
    println!("=== dependency_parser detail ===");
    for n in names.iter().filter(|n| n.starts_with("dependency_parser.")) {
        println!("  {n} -> {:?}", cp.get(n)?.dims());
    }
    println!();
    println!("=== tagger heads ===");
    let heads = ["pos_tagger", "subpos_tagger", "reading_tagger", "conjtype_tagger",
                 "conjform_tagger", "word_feature_tagger", "ne_tagger",
                 "base_phrase_feature_tagger", "dependency_parser", "dependency_type_parser",
                 "cohesion_analyzer", "discourse_relation_analyzer"];
    for h in heads {
        let count = names.iter().filter(|n| n.starts_with(&format!("{h}."))).count();
        println!("  {h}: {count} tensors");
    }
    Ok(())
}
