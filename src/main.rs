use std::path::PathBuf;
use rustmetamap::{MetamapLite, Metamap, MetamapModel, MetamapPool};
use futures::stream::{self, StreamExt};
use structopt::{StructOpt};

// TODO
// -> MetamapLite
// -> Multiprocess Pool ?!

fn main() {
    //let mm = Metamap::new(&PathBuf::from("/home/vin/public_mm/bin/metamap20"));

    // let mm = Metamap::new(&PathBuf::from("/home/vin/public_mm_lite/metamaplite.sh"));

    let mm = MetamapPool::new(&PathBuf::from("/home/vin/public_mm/bin/metamap20"), 2);

    let sentences = vec![
        "This man had a heart attack"; 100
    ];

    //let sentences: Vec<&&str> = sentences.iter().cycle().take(10).collect();

    mm.extract_concepts(sentences, false);

    // let outputs: Vec<_> = sentences
    //     .iter()
    //     .map(|s| mm.extract_concepts(s, false))
    //     .collect();

    //println!("{:?}", outputs);
}