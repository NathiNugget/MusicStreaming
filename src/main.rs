use std::{fs, io};

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{File, ReadDir};
use std::io::{BufReader, Error, ErrorKind, Write};
use std::num::ParseIntError;

fn print_songs(songs: &Vec<String>) -> Result<bool, Error> {
    match songs.len() {
        0 => {
            println!("Ingen sange fundet på computeren, desværre");
            Ok(false)
        }

        _ => {
            for song in songs {
                println!(
                    "#{}: {}",
                    songs.iter().position(|x| song == x).unwrap() + 1,
                    song
                );
            }
            Ok(true)
        }
    }
}

fn main() {
    let songs = read_songs();
    let playlist: Vec<String> = Vec::new();


    match &songs {
        Ok(entries) => match entries {
            None => {}
            Some(s) => {
                let _ = print_songs(&s);
                play_song(&s);
            }
        },
        _ => {}
    }




    fn read_number() -> u8 {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);

        loop {
            let res = buf.trim().parse::<u8>();
            match res {
                Ok(val) => {
                    return val;
                }
                Err(_) => {
                    println!("Please enter a positive number within range 1-255");
                    buf.clear();
                    continue;
                }
            }
        }
    }

    fn play_song(songs: &Vec<String>) {
        if songs.len() == 0 {
            panic!("Ingen sange var fundet på computeren, applikation lukker nu");
        }
        let size = 0;

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file = BufReader::new(File::open(songs.get(size).unwrap()).unwrap());

        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.play();
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        sink.pause();
    }

    fn read_songs() -> Result<Option<Vec<String>>, Error> {
        let mut track_paths: Vec<String> = Vec::new();
        let file = fs::read_dir("music")?;

        for song in file {
            match song {
                Ok(entry) => track_paths.push(entry.path().display().to_string()),
                Err(_) => return Err(Error::new(ErrorKind::NotFound, ("File could not be read"))),
            }
        }
        Ok(Some(track_paths))
    }
