use std::fs;
use std::path::Path;

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

fn playlist<R>(root: R) -> std::io::Result<()>
where
    R: AsRef<Path>,
{
    let mut playlist = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?.path();
        let path = entry.clone().as_path();
        if path.is_mp3().is_some() {
            playlist.push(path.to_str().unwrap());

            dbg!(path);
        }
    }
    dbg!(&playlist);
    Ok(())
}

//fn play<P>(playlist: P)
//where
//    P: AsRef<PathBuf> + std::iter::Iterator,
//    <P as Iterator>::Item: AsRef<Path>,
//{
//    for audio in playlist {
//        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//        let sink = rodio::Sink::try_new(&handle).unwrap();
//
//        let file = std::fs::File::open(audio).unwrap();
//        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
//
//        println!("now playing: ");
//        sink.sleep_until_end();
//    }
//}

fn main() -> std::io::Result<()> {
    let x = Path::new("/Users/ethan/Music/LiquidDNB");
    let playlist = playlist(x);
    Ok(())
}
