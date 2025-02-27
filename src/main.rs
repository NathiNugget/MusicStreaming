mod song_lib;
mod user_input;
mod music_player;
use rodio::{Source};
use std::io::{Write};
use song_lib::read_songs;
use music_player::initialize_player;
use song_lib::print_songs;



fn main() {
    let songs = read_songs();

    match &songs {
        Ok(entries) => match entries {
            None => {println!("kunne ikke finde nogen sange");},
            Some(s) => {
                let _res = print_songs(&s);
                match _res {

                    Ok(true) => {
                        let play_res = initialize_player(&s);
                        if let Err(e) = play_res {
                            println!("error med afspiling (play_song()) og ikke at finde sangen: {}", e);
                        }
                    }

                    Ok(false) => {

                        println!("ingen sange tilgængelige: ");
                    }

                    Err(e) => {
                        println!("fejl med at vise sange: {}", e);
                    }
                }
            }
        },
        Err(_) => {
            println!("error HTTP status code 500 lowkey"); // fanger songs error/fejlen
        }
    }
}