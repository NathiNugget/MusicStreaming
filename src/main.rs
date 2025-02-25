use std::{fs, io, thread};

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{read_link, File, ReadDir};
use std::io::{read_to_string, BufReader, Error, ErrorKind, Write};
use std::num::ParseIntError;
use std::time::Duration;
use rodio::decoder::DecoderError;

fn print_songs(songs: &Vec<String>) -> Result<bool, Error> {
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

fn main() {
    let songs = read_songs();

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




    fn read_number() -> u16 {
        let mut buf = String::new();


        loop {
            let _ = io::stdin().read_line(&mut buf);
            let res = buf.trim().parse::<u16>();
            match res {
                Ok(val) => {
                    match val {
                        0 => {return val},
                        _ => return val
                    }
                }
                Err(_) => {
                    println!("Venligst skriv en værdi indenfor rækkevidden 1-255");
                    buf.clear();
                    continue;
                }
            }
        }
    }

    fn play_song(songs: &Vec<String>) ->  Result<(), Error> {
        if songs.len() == 0 {
            panic!("Ingen sange var fundet på computeren, applikation lukker nu");
        }

        let internet = true;
        let mut attempts:u8 = 3;

        fn simulate_internet(attempts: &mut u8, sink: &mut Sink){
            if *attempts > 0 {
                *attempts -= 1;
            }
            else {
                println!("Ingen internet-forbindelse, applikationen venter på at oprette forbindelse..");
                if !sink.is_paused() {
                    sink.pause();
                }
                thread::sleep(Duration::from_millis(1500));
                *attempts = 3;
                println!("Forbindelse genoprettet, applikation fortsætter");
                sink.play();


            }

        }



        fn add_to_sink(idx :u16, songs :&Vec<String>) -> Result<Option<Decoder<BufReader<File>>>, Error> {
            let idx = (idx-1) as usize;
            let b_reader = BufReader::new(File::open(songs.get(idx).unwrap())?);
            let source = match Decoder::new(b_reader) {
                Ok(x) => x,
                Err(e) => return Err(Error::new(ErrorKind::Other, e)),

            };
            Ok(Option::from(source))

        }

        println!("Skriv et tal for at spille en af disse sange");




        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sink = Sink::try_new(&stream_handle).unwrap();
        let source = add_to_sink(read_number(), &songs)?;
        match source {
            None => println!("Ingen fil fundet"),
            Some(val) => sink.append(val),
        }
        sink.play();
        let mut buf = String::new();
        loop {
            println!("Vælg en af nedenstående options:\n-> 'add' for at tilføje en sang til playlisten\n-> 'P' for at pause playback\n-> stop for at stoppe og rydde playlisten\n-> 'R' for at fortsætte playback\n-> break for at spille til playlisten er tom");
            buf.clear();
            let _ = io::stdin().read_line(&mut buf);
            match buf.trim().to_lowercase() {
                s if s.contains("add") => {
                    println!("Skriv tallet for den sang du gerne vil tilføje til playlisten");
                    print_songs(&songs)?;
                    if let Some(song) = add_to_sink(read_number(), &songs)?{
                        simulate_internet(&mut attempts, &mut sink);
                        sink.append(song);
                    } else {
                        println!("Noget gik galt; sang ikke tilføjet")
                    }

                },

                s if s.contains("p") => sink.pause(),
                s if s.contains("stop") => sink.stop(),
                s if s.contains("r") => {
                    sink.play();
                    simulate_internet(&mut attempts, &mut sink);
                },
                s if s.contains("break") => break,
                s if s.contains("exit") => return Ok(()),

                String { .. } => {}
            }

        }
        sink.sleep_until_end();
        Ok(())



    }

    fn read_songs() -> Result<Option<Vec<String>>, Error> {
        let mut track_paths: Vec<String> = Vec::new();
        let file = fs::read_dir("music")?;

        for song in file {
            match song {
                Ok(entry) => track_paths.push(entry.path().display().to_string()),
                Err(_) => return Err(Error::new(ErrorKind::NotFound, "File could not be read")),
            }
        }
        Ok(Some(track_paths))
    }
}