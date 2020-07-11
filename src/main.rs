use std::path::PathBuf;
use rustmetamap::Metamap;

fn main() {
    let mut mm = Metamap::new(&PathBuf::from("/home/vin/Downloads/public_mm/bin/metamap20"));
    let outputs = mm
                    .extract_concepts("This man is having a heart attack. He is shaking. Call an ambulance", true);

    println!("{:?}", outputs);
}