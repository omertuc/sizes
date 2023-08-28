use clap::Parser;
use image::io::Reader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(long, default_value_t = 300 as usize)]
    pub(crate) ppi: usize,

    #[arg(long)]
    pub(crate) files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for file in args.files {
        println!("File: {:?}", file);
        let path = Path::new(file.as_path());
        let reader = Reader::open(path).unwrap();
        let dimensions = reader.into_dimensions().unwrap();
        println!("Dimensions: {:?}", dimensions);

        // Real size
        println!(
            "Real size: {:.2} x {:.2} cm",
            dimensions.0 as f64 / args.ppi as f64 * 2.54,
            dimensions.1 as f64 / args.ppi as f64 * 2.54
        );
    }
}
