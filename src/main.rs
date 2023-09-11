////use rodio::{Decoder, OutputStream};
//use std::fs::File;
//use std::io::{BufReader, Result};
//use std::path::Path;
//
//#[allow(unused_variables)]
//fn main() -> Result<()> {
//    let root = Path::new("/Users/ethan/Music/liquid/classic_full_cycle.mp3");
//    let (_stream, handle) = OutputStream::try_default().unwrap();
//    let file = BufReader::new(File::open(root).unwrap());
//    let source = Decoder::new(file).unwrap();
//
//    Ok(())
//}

use rand::prelude::*;
use rodio::Sink;
use std::path::Path;
use std::{fs, io};

fn initialize_playlist<R>(root: R) -> Result<Vec<String>, io::Error>
where
    R: AsRef<Path>,
{
    let mut playlist: Vec<String> = Vec::new();
    for file in fs::read_dir(root)? {
        let _entry = file?.path();
        let entry = _entry.as_path().to_str().unwrap().to_string();
        if entry.ends_with(".mp3") {
            playlist.push(entry);
        } else if Path::new(&entry).is_dir() {
            playlist.push(entry);
        }
    }
    Ok(playlist)
}

#[allow(dead_code)]
fn play_default(playlist: &mut Vec<String>, sink: &Sink) {
    playlist.sort();
    for audio in playlist.iter() {
        let file = fs::File::open(audio).unwrap();
        sink.append(rodio::Decoder::new(io::BufReader::new(file)).unwrap());

        let audio_name = Path::new(audio).file_name().unwrap().to_str().unwrap();
        println!("now playing: {audio_name}");
        sink.sleep_until_end();
    }
}

#[allow(dead_code)]
fn play_shuffle(playlist: &mut Vec<String>, sink: &Sink) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    playlist.shuffle(&mut rng);

    for audio in playlist.iter() {
        if Path::new(audio).is_dir() {
            let mut album = initialize_playlist(Path::new(audio))?;
            play_default(&mut album, sink);
        }
        let file = fs::File::open(audio).unwrap();
        let audio_name = Path::new(audio).file_name().unwrap().to_str().unwrap();
        sink.append(rodio::Decoder::new(io::BufReader::new(file)).unwrap());
        println!("now playing: {audio_name}");
        sink.sleep_until_end();
    }

    Ok(())
}

//TODO: fix this cursed function
fn show_tracks(playlist: &mut Vec<String>, sink: &Sink) -> io::Result<()> {
    playlist.sort();
    for audio in playlist.iter() {
        if Path::new(audio).is_dir() {
            let album_path = Path::new(audio);
            let mut album = initialize_playlist(album_path)?;
            println!(
                "ALBUM: {}",
                album_path.file_name().unwrap().to_str().unwrap()
            );
            show_tracks(&mut album, sink)?;
        }
        println!(
            "{}",
            Path::new(audio).file_name().unwrap().to_str().unwrap()
        );
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = Path::new("/Users/ethan/Music/liquid");
    let mut playlist = initialize_playlist(root)?;

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    //let _ = play_shuffle(&mut playlist, &sink);
    //let _ = show_tracks(&mut playlist, &sink);

    Ok(())
}
