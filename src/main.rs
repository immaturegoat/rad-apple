use cascii::{AsciiConverter, VideoOptions, ConversionOptions};
use std::path::Path;
use std::fs::{self, File};
use std::io::Read;
use terminal_size::{Width, Height, terminal_size};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {    

    let mut frames: Vec<String> = read_files()?;

    frames.sort();
    play_animation(frames)?;
    // println!("{}", frames[0]);
    Ok(())
}

fn get_frames() -> Result<(), Box<dyn std::error::Error>> {
    let converter = AsciiConverter::new();
    
    let size = terminal_size();
    let mut width: u32 = 80;
    let mut height = 0;
    if let Some((Width(w), Height(h))) = size {
        width = u32::from(w);
    } else {
        println!("unable to get terminal size");
    }

    let video_options = VideoOptions {
        fps: 30,
        start: Some("0".to_string()),
        end: Some("10".to_string()),
        columns: width,
        extract_audio: false,
        preprocess_filter: None,
    };

    let conversion_options = ConversionOptions::default().with_font_ratio(0.2).with_luminance(20).with_columns(width);

    converter.convert_video(Path::new("assets/apple.mp4"), Path::new("output_frames"), &video_options, &conversion_options, false)?;

    Ok(())
}

fn read_files() -> std::io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir("output_frames/")? {
        let entry = entry?;
        let path = entry.path();

        let content = fs::read_to_string(path);
        let frame = match content {
            Ok(val) => val,
            Err(err) => format!("error: {}", err),
        };

        files.push(frame);
    }
    Ok(files)
}

fn play_animation(frames: Vec<String>) -> io::Result<()> {
    let frame = 0;
    let mut stdout = stdout();
    execute!(stdout, Hide, Clear(ClearType::All))?;
    for frame in &frames {
        execute!(stdout, MoveTo(0, 0));
        println!("{}", frame);
        thread::sleep(Duration::from_millis(33));
    }

    execute!(stdout, Show)?;
    Ok(())
}
