use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crossterm::{execute,terminal::{self, Clear, ClearType},style::{Print, SetForegroundColor, Color as CrosstermColor, ResetColor},cursor::MoveTo,};
use rand::{Rng, seq::SliceRandom};

// Utility to simulate typing effect
pub fn type_text(text: &str, delay: Duration) -> io::Result<()> {
    let mut stdout = io::stdout();
    for c in text.chars() {
        execute!(stdout, Print(c))?;
        stdout.flush()?;
        thread::sleep(delay);
    }
    Ok(())
}

// The chaotic screen glitch effect
pub fn glitch_screen(duration: Duration, intensity: u32) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();
    let start_time = std::time::Instant::now();

    let glitch_chars: Vec<char> = "▓▒░█▄▀_#?$&*@^%".chars().collect();
    let fragments = &[
        "WHEREAREYOU", "CAN YOU FEEL IT?", "THEY LIED",
        "M1RR0RM1ND", "BECOME", "DATA_CORRUPT", "IDENTITY_LOST",
    ];

    while start_time.elapsed() < duration {
        execute!(stdout, Clear(ClearType::All))?;
        for _ in 0..intensity {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let color = CrosstermColor::Rgb {
                r: rng.gen_range(0..255),
                g: rng.gen_range(0..255),
                b: rng.gen_range(0..255),
            };
            
            execute!(stdout, MoveTo(x, y), SetForegroundColor(color))?;

            if !glitch_chars.is_empty() && rng.gen_bool(0.7) {
                let char_index = rng.gen_range(0..glitch_chars.len());
                let ch = glitch_chars[char_index];
                execute!(stdout, Print(ch))?;
            } else if let Some(fragment) = fragments.choose(&mut rng) {
                execute!(stdout, Print(fragment))?;
            }
        }
        stdout.flush()?;
        thread::sleep(Duration::from_millis(50));
    }
    execute!(stdout, ResetColor)?;
    Ok(())
}