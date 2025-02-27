use crate::song_lib::print_songs;
use crate::user_input::read_number;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

pub fn initialize_player(songs: &Vec<String>) -> Result<(), Error> {
    if songs.len() == 0 {
        panic!("Ingen sange var fundet på computeren, applikation lukker nu");
    }

    let mut attempts:u8 = 3;

    fn simulate_internet(attempts: &mut u8, sink: &mut Sink) {
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

    fn choose_song(idx :u16, songs :&Vec<String>) -> Result<Option<Decoder<BufReader<File>>>, Error> {
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
    let mut sink_ref = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let source = choose_song(read_number(), &songs)?;
    match source {
        None => println!("Ingen fil fundet"),
        Some(val) => {
            let mut sink = sink_ref.lock().unwrap();
            sink.append(val);
        },
    }

    let mut buf = String::new();
    loop {
        println!("Vælg en af nedenstående options:\n-> 'add' for at tilføje en sang til playlisten\n-> 'P' for at pause playback\n-> stop for at stoppe og rydde playlisten\n-> 'R' for at fortsætte playback\n-> break for at spille til playlisten er tom");
        buf.clear();
        let _ = io::stdin().read_line(&mut buf);
        let mut sink = sink_ref.lock().unwrap();  // Lock here to avoid multiple locks

        match buf.trim().to_lowercase() {
            s if s.contains("add") => {
                println!("Skriv tallet for den sang du gerne vil tilføje til playlisten");
                print_songs(&songs)?;
                if let Some(song) = choose_song(read_number(), &songs)?{
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
            _ => {}
        }
    }
    sink_ref.lock().unwrap().sleep_until_end();
    Ok(())
}
