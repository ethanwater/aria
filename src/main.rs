use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

trait Mp3 {
    fn is_mp3(&self) -> Option<()>;
}

impl Mp3 for Path {
    fn is_mp3(&self) -> Option<()> {
        let path = self.file_name()?.to_str()?;
        if path.ends_with(".mp3") {
            return Some(());
        } else {
            return None;
        }
    }
}

fn playlist<R>(root: R) -> std::io::Result<PathBuf>
where
    R: AsRef<Path>,
{
    let mut playlist = PathBuf::new();
    for entry in fs::read_dir(&root)? {
        let entry = entry?.path();
        let path = entry.as_path();
        if path.is_mp3().is_some() {
            playlist.push(path);
            dbg!(path);
        }
    }

    Ok(playlist)
}

fn play<P>(playlist: P)
where
    P: AsRef<PathBuf> + std::iter::Iterator,
    <P as Iterator>::Item: AsRef<Path>,
{
    for audio in playlist {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = std::fs::File::open(audio).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        println!("now playing: ");
        sink.sleep_until_end();
    }
}

fn main() -> std::io::Result<()> {
    let x = Path::new("/Users/ethan/Music/LiquidDNB");
    let playlist = playlist(x);
    play(playlist)?;
}
