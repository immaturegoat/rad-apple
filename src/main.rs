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
use std::io::BufReader;

fn main() -> std::io::Result<()> {    
    fs::remove_dir_all("output_frames");
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
    let video_path = "assets/apple.mp4";
    
    let duration_result= get_video_length(video_path);
    let seconds = match duration_result {
        Ok(seconds) => seconds,
        Err(e) => {
            println!("can't read video");
            "0".to_string()
        }
    };

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
        end: Some(seconds),
        columns: width,
        extract_audio: false,
        preprocess_filter: None,
    };

    let conversion_options = ConversionOptions::default().with_font_ratio(0.2).with_luminance(20).with_columns(width);

    converter.convert_video(Path::new(video_path), Path::new("output_frames"), &video_options, &conversion_options, false)?;

    Ok(())
}

fn get_video_length(path: &str) -> Result<String, Box<dyn std::error::Error>>{
    let file = File::open(path)?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    
    let duration_secs = mp4.duration().as_secs();
    Ok(duration_secs.to_string())
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
