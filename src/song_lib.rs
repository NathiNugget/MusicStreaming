use std::fs;
use std::io::{Error, ErrorKind};

pub fn read_songs() -> Result<Option<Vec<String>>, Error> {
    let mut track_paths: Vec<String> = Vec::new();
    let file = fs::read_dir("music")?;

    for song in file {
        match song {
            Ok(entry) => track_paths.push(entry.path().display().to_string()),
            Err(_) => return Err(Error::new(ErrorKind::AddrInUse, "Some file could not be read")),
        }
    }
    Ok(Some(track_paths))
}

pub fn print_songs(songs: &Vec<String>) -> Result<bool, Error> {
    match songs.len() {
        0 => {
            println!("Ingen sange fundet på computeren, desværre");
            Ok(false)
        }

        _ => {
            for song in songs {
                let title = song.trim().replace("music\\", "").replace(".mp3","").replace("_"," ");
                println!(
                    "#{}: {}",
                    songs.iter().position(|x| song == x).unwrap() + 1,
                    title
                );
            }
            Ok(true)
        }
    }
}
