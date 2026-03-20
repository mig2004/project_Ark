use std::time::{Instant, Duration};
use scrap::{Display, Capturer};
use std::io::{self, Write, ErrorKind};
use std::fs::File;
use crossterm::event::{self, Event, KeyCode};

unsafe extern "C" {
    fn serpent_hound_bite(input: *const u8, keys: *const u8, output: *mut u8);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::primary().map_err(|_| "Display not found")?;
    let mut capturer = Capturer::new(display).map_err(|_| "Capturer failed")?;
    
    let input = [0u8; 32];
    let keys = [0u8; 528];
    let mut output = [0u8; 32];

    println!("--- ARKMULE STABILIZED TEST (LUBUNTU) ---");
    println!("PRESS 'Q' TO QUIT | 'S' FOR SNAPSHOT");

    loop {
        let start = Instant::now();
        unsafe { serpent_hound_bite(input.as_ptr(), keys.as_ptr(), output.as_mut_ptr()); }
        let kinetic_time = start.elapsed().as_nanos();

        // Standard Output instead of Alternate Screen
        print!("\r🚀 Core: {}ns | Status: ACTIVE    ", kinetic_time);
        io::stdout().flush()?;

        // 🛡️ Heavy polling to bypass terminal lag
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        println!("\n[!] Exit Signal Received.");
                        break;
                    },
                    KeyCode::Char('s') => {
                        match capturer.frame() {
                            Ok(frame) => {
                                let mut file = File::create("snapshot.raw")?;
                                file.write_all(&frame)?;
                                println!("\n[+] SNAPSHOT SAVED ({} bytes)", frame.len());
                            },
                            Err(e) => println!("\n[!] Capture Sync: {:?}", e.kind()),
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
