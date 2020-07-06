use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(name = "Metamap Location", parse(from_os_str))]
    pub metamap_location: PathBuf,
}

fn main() {
    let opts = Opt::from_args();
}