use cascii::{AsciiConverter, VideoOptions, ConversionOptions};
use std::path::Path;
use std::fs::{self, File};
use terminal_size::{Width, Height, terminal_size};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {    
    if !Path::new("output_frames").is_dir() {
        get_frames();
        
    }
    let frames: Vec<String> = read_files()?;

    play_animation(frames)?;

    fs::remove_dir_all("output_frames");
    Ok(())
}

fn get_frames() -> Result<(), Box<dyn std::error::Error>> {
    let converter = AsciiConverter::new();
    
    let size = terminal_size();
    let mut width: u32 = 80;
    if let Some((Width(w), Height(_h))) = size {
        width = u32::from(w);
    } else {
        println!("unable to get terminal size");
    }

    let video_options = VideoOptions {
        fps: 30,
        start: Some("0".to_string()),
        end: Some("30".to_string()),
        columns: width,
        extract_audio: false,
        preprocess_filter: None,
    };

    let conversion_options = ConversionOptions::default().with_font_ratio(0.2).with_luminance(20).with_columns(width);

    converter.convert_video(Path::new("assets/apple.mp4"), Path::new("output_frames"), &video_options, &conversion_options, false)?;

    Ok(())
}

fn read_files() -> std::io::Result<Vec<String>> {
    let mut files  = Vec::new();

    for file in fs::read_dir("output_frames/").unwrap() {
        files.push(file.unwrap().path().display().to_string());
    }

    files.sort();

    let mut frames: Vec<String> = Vec::new();
    for file in files {
        let content = fs::read_to_string(file);
        let frame = match content {
            Ok(val) => val,
            Err(err) => format!("error: {}", err),
        };

        frames.push(frame);
    }
    Ok(frames)
}

fn play_animation(frames: Vec<String>) -> io::Result<()> {
    let mut stdout = stdout();
    let mut lock = stdout.lock();

    execute!(stdout, Hide, Clear(ClearType::All))?;    
    for frame in &frames {
        execute!(stdout, MoveTo(0, 0));
        execute!(stdout, Clear(ClearType::FromCursorDown));
        writeln!(lock, "{}", frame);
        thread::sleep(Duration::from_millis(33));
    }

    execute!(stdout, Show)?;
    Ok(())
}
