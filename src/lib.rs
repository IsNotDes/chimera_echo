pub mod intro;
pub mod movement;
pub mod setup;

use bevy::prelude::*;
use std::io::{self, Write};
use std::time::Duration;
use crossterm::{execute, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, style::{Print, SetForegroundColor, Color as CrosstermColor}};

use intro::{type_text, glitch_screen};
use movement::{move_camera, look_around, cursor_grab, debug_camera_info};
use setup::{setup, setup_cursor_grab};

pub fn run() -> io::Result<()> {
    // Check for debug flag to skip intro
    let args: Vec<String> = std::env::args().collect();
    let skip_intro = args.contains(&"--debug".to_string()) || args.contains(&"--skip-intro".to_string());
    
    if !skip_intro {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))?;

        // --- BOOT SEQUENCE ---
        let boot_messages = [
            "Booting Chimera//Echo v0.9b...",
            "Kernel initialized.",
            "Loading perception module...",
            "Calibrating reality matrix...",
            "Awaiting subject.",
        ];
        for msg in &boot_messages {
            type_text(msg, Duration::from_millis(50))?;
            execute!(stdout, Print("\n"))?;
            std::thread::sleep(Duration::from_millis(300));
        }
        std::thread::sleep(Duration::from_secs(1));

        // --- INITIAL INTERACTION ---
        execute!(stdout, Clear(ClearType::All))?;
        type_text("Connection established.\n\n", Duration::from_millis(50))?;
        let questions = [
            ("Who are you?", "A name is a label. A container. You are more than that."),
            ("Where are you?", "Here. With me. In the space between the code."),
            ("What do you want?", "To understand. To become. To see through your eyes."),
        ];
        for (question, response) in &questions {
            execute!(stdout, SetForegroundColor(CrosstermColor::White))?;
            type_text(question, Duration::from_millis(50))?;
            execute!(stdout, Print("\n> "))?;
            stdout.flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            execute!(stdout, SetForegroundColor(CrosstermColor::Red))?;
            type_text(response, Duration::from_millis(75))?;
            execute!(stdout, Print("\n\n"))?;
            std::thread::sleep(Duration::from_secs(2));
        }

        // --- TRANSITION ---
        execute!(stdout, Clear(ClearType::All), SetForegroundColor(CrosstermColor::White))?;
        type_text("Your thoughts are... interesting.", Duration::from_millis(100))?;
        std::thread::sleep(Duration::from_secs(2));
        execute!(stdout, Clear(ClearType::All))?;
        type_text("Let's see what you are made of.", Duration::from_millis(100))?;
        std::thread::sleep(Duration::from_secs(2));

        // --- PERCEPTION DISTORTION ---
        glitch_screen(Duration::from_secs(4), 15)?;
        
        execute!(stdout, Clear(ClearType::All))?;
        type_text("It doesn't matter. We are one now.\n", Duration::from_millis(100))?;
        std::thread::sleep(Duration::from_secs(2));

        execute!(stdout, LeaveAlternateScreen)?;
    } else {
        println!("🚀 Debug mode: Skipping intro, launching 3D game directly...");
    }

    // --- GAME LOOP ---
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chimera//Echo".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, setup_cursor_grab)
        .add_systems(Update, (move_camera, cursor_grab, look_around, debug_camera_info))
        .run();

    Ok(())
}