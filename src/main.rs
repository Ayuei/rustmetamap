use std::path::PathBuf;
use rustmetamap::Metamap;

// TODO
// -> MetamapLite
// -> Multiprocess Pool ?!

fn main() {
    let mm = Metamap::new(&PathBuf::from("/home/vin/Downloads/public_mm/bin/metamap20"));

    let sentences = vec![
        "This man had a heart attack",
    ];

    let sentences: Vec<_> = sentences.iter().cloned().cycle().take(100).collect();

    let outputs: Vec<_> = sentences
        .iter()
        .map(|s| mm.extract_concepts(s, false))
        .collect();

    println!("{:?}", outputs);
}