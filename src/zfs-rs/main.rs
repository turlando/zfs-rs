use clap::Parser;
use std::path::PathBuf;
use zfs::vdev::physical::Physical;
use zfs::vdev::label::LabelNumber;

#[derive(Debug, Parser)]
#[command(about, author, version, long_about = None)]
struct Args {
    #[arg(short, long)]
    device: PathBuf,
}

fn main() {
    let args = Args::parse();

    let vdev = Physical::open(args.device).unwrap();
    let label_nvlist = zfs::vdev::label::read_nvlist(&vdev, LabelNumber::L0);

    println!("{:?}", vdev);
    println!("{:?}", label_nvlist);
}
