use std::io;

pub fn read_number() -> u16 {
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