// This app uses the ELM architecture
// https://ratatui.rs/concepts/application-patterns/the-elm-architecture.html
use super::model::Presentation;
use ratatui::{
    prelude::{Alignment, CrosstermBackend, Terminal},
    widgets::{block::Title, Block, Borders, Paragraph},
};

pub type Frame<'a> = ratatui::Frame<'a>;

// MESSAGES
#[derive(PartialEq)]
enum Message {
    Next,
    Previous,
    Reset,
    Quit,
}

// UPDATE
fn update(presentation: &mut Presentation, msg: Message) -> Option<Message> {
    match msg {
        Message::Next => {
            if presentation.slide_index < presentation.slides.len() - 1 {
                presentation.slide_index += 1;
            }
        }
        Message::Previous => {
            if presentation.slide_index > 1 {
                presentation.slide_index -= 1;
            }
        }
        Message::Reset => presentation.reset(),
        Message::Quit => presentation.should_quit = true, // We can handle cleanup and exit here
    };
    None
}

// VIEW
fn view(presentation: &mut Presentation, frame: &mut Frame) {
    let area = frame.size();
    let block = Block::new()
        .borders(Borders::ALL)
        .title(Title::from("Presento").alignment(Alignment::Center));
    let paragraph = Paragraph::new(presentation.content());
    frame.render_widget(paragraph.block(block), area);
}

// Convert Event to Message
// We don't need to pass in a `presentation` (model) to this function in this example
// but you might need it as your project evolves
fn handle_event(_: &Presentation) -> std::io::Result<Option<Message>> {
    let message = if crossterm::event::poll(std::time::Duration::from_millis(250))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            match key.code {
                crossterm::event::KeyCode::Char('j') => Message::Next,
                crossterm::event::KeyCode::Char('k') => Message::Previous,
                crossterm::event::KeyCode::Char('q') => Message::Quit,
                _ => return Ok(None),
            }
        } else {
            return Ok(None);
        }
    } else {
        return Ok(None);
    };
    Ok(Some(message))
}

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn run(mut presentation: Presentation) -> std::io::Result<()> {
    initialize_panic_handler();

    // Startup
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        // Render the current view
        terminal.draw(|f| {
            view(&mut presentation, f);
        })?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&presentation)?;

        // Process updates as long as they return a non-None message
        while current_msg != None {
            current_msg = update(&mut presentation, current_msg.unwrap());
        }

        // Exit loop if quit flag is set
        if presentation.should_quit {
            break;
        }
    }

    // Shutdown
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
