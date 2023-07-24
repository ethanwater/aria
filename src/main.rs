use std::io::BufReader;
const MUSIC_DIR: &str = "/Users/ethan/Music/Music/Media.localized/Music";

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("/Users/ethan/Music/Music/Media.localized/Music/Nuffbonsai/Nuffbonsai Mixes/01 Classic Full Cycle.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    println!("now playing: ");
    sink.sleep_until_end();
}
