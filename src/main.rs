use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use presento::model::{Presentation, Slide};

#[derive(Debug, Parser)]
struct Cli {
    /// Input file to read
    path: PathBuf,
}

fn parse_presentation(path: PathBuf) -> Presentation {
    let mut slides = Vec::new();
    let mut current_slide = Vec::new();
    for (line_number, line) in read_to_string(path.clone()).unwrap().lines().enumerate() {
        if line.to_string().trim() == "---" {
            if current_slide.is_empty() {
                if line_number != 0 {
                    println! {"Warning: Ending empty slide at line {:?}", line_number};
                }
            } else {
                slides.push(Slide::from_lines(&current_slide));
                current_slide = Vec::new();
            }
        } else {
            current_slide.push(line.to_string());
        }
    }
    // add last slide to slides
    if !current_slide.is_empty() {
        slides.push(Slide::from_lines(&current_slide));
    }
    println! {"Found {:?} Slides", slides.len()}
    Presentation::from_slides(slides)
}

fn main() {
    let args = Cli::parse();
    let presentation = parse_presentation(args.path);
    let _ = presento::app::run(presentation);
}
