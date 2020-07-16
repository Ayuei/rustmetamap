use std::path::PathBuf;
use rustmetamap::{MetamapLite, Metamap, MetamapModel};
use futures::stream::{self, StreamExt};
use structopt::{StructOpt};

// TODO
// -> MetamapLite
// -> Multiprocess Pool ?!

fn main() {
    //let mm = Metamap::new(&PathBuf::from("/home/vin/public_mm/bin/metamap20"));

    let mm = MetamapLite::new(&PathBuf::from("/home/vin/public_mm_lite/metamaplite.sh"));

    let sentences = vec![
        "This man had a heart attack",
    ];

    let sentences: Vec<&&str> = sentences.iter().cycle().take(10).collect();

    let outputs: Vec<_> = sentences
        .iter()
        .map(|s| mm.extract_concepts(s, false))
        .collect();

    //println!("{:?}", outputs);
}