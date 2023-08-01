use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::{BufReader, Result};
use std::path::Path;

#[allow(unused_variables)]
fn main() -> Result<()> {
    let root = Path::new("/Users/ethan/Music/liquid/classic_full_cycle.mp3");
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(root).unwrap());
    let source = Decoder::new(file).unwrap();

    Ok(())
}
