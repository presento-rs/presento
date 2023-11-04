pub struct Slide {
    pub states: Vec<String>,
    pub state_idx: usize,
}
impl Slide {
    pub fn from_lines(lines: &Vec<String>) -> Self {
        Self {
            state_idx: 0,
            states: vec![lines.join("\n")],
        }
    }

    /// Get the current content of the slide considering internal state
    pub fn content(self: &Self) -> &String {
        &self.states[self.state_idx]
    }

    /// Reset all state within the current slide
    pub fn reset(&mut self) {
        self.state_idx = 0;
    }
}

// MODEL for the MVC
pub struct Presentation {
    pub slide_index: usize,
    pub slides: Vec<Slide>,
    pub should_quit: bool,
}

impl Presentation {
    pub fn from_slides(slides: Vec<Slide>) -> Self {
        Presentation {
            slide_index: 0,
            slides: slides,
            should_quit: false,
        }
    }
    /// Get the current content of the presentation considering internal state
    pub fn content(&self) -> String {
        self.slides[self.slide_index].content().to_string()
    }

    /// Reset all state and start over again
    pub fn reset(&mut self) {
        self.slide_index = 0;
        self.slides.iter_mut().for_each(|slide| slide.reset());
    }
}
