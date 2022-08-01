use clap::Parser;
use std::path::PathBuf;
use crate::importer::*;

/// Build voxel logic circuits to execute
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Cli {

    /// Input file to load to build a logic circuit
    #[clap(short, long, parse(from_os_str))]
    input_file: PathBuf,

}


fn parse_args() {
    let args = Cli::parse();

    let file_path = args.input_file;

    // test the file extension
    let schema = match file_path.extension() {
        Some("blc")  => {
            Schema::load(file_path)
        },
        Some("xraw") => {
            let matrix = xraw::load_file(file_path);
        },
        Some("vox")  => Err(ErrorFile::Unknown),
        None         => Err(ErrorFile::Unknown),
    };

    let schema = match result {
        Ok(s) => {
            if let Err(errors) = s.verify() {
                // TODO: do something with errors
                return;
            }
            s
        },
        Err(_) => {

        },
    }
}