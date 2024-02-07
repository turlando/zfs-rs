use clap::Parser;
use std::path::PathBuf;
use zfs::vdev::physical::Physical;

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = None)]
struct Args {
    #[arg(short, long)]
    device: PathBuf,
}

fn main() {
    let args = Args::parse();
    let vdev = Physical::open(args.device).unwrap();
    println!("{:?}", vdev);
}
