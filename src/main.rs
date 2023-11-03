use clap::Parser;
use std::{fs, fs::read_to_string, path::PathBuf};

struct Presentation {
    slides: Vec<Slide>,
}

struct Slide {
    ctr: i32,
    content: Vec<String>,
}

impl Slide {
    pub fn from_lines(lines: &Vec<String>) -> Self {
        Self {
            ctr: 0,
            content: vec![lines.join("\n")],
        }
    }
}

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
    Presentation { slides: slides }
}

fn main() {
    let args = Cli::parse();
    let presentation = parse_presentation(args.path);
    let content = presentation.slides[1].content[0].as_str();
    println! { "Content: {:?}", content}
    println! {"------------------------------"}
    termimad::print_inline(&content);
}
